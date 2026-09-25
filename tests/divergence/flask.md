Score(3000)=0.676 I=0.885 C=0.516 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.771/0.637/0.694/0.676/0.559/0.515/0.524

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 0.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 80 | 27 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| walker |  | 89 | 9 | listing of 'examples' |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| walker |  | 100 | 11 | listing of '.devcontainer' |  |  | 0.000 |
| walker |  | 113 | 13 | listing of '.github' |  |  | 0.000 |
| walker |  | 136 | 23 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 147 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.488 |
| walker |  | 238 | 102 | README headline in README.md |  |  | 1.000 |
| walker |  | 268 | 30 | headings outline in README.md |  |  | 1.000 |
| ns | 271 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.592 |
| walker |  | 361 | 93 | listing of 'src/flask' |  |  | 0.836 |
| walker |  | 375 | 14 | listing of 'src/flask/json' |  |  | 0.903 |
| walker |  | 392 | 17 | listing of 'src/flask/sansio' |  |  | 1.000 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.914 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.782 |
| walker |  | 632 | 240 | python names src/flask/__init__.py |  |  | 0.797 |
| walker |  | 647 | 15 | python names src/flask/blueprints.py |  |  | 0.797 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.813 |
| walker |  | 796 | 149 | python names src/flask/json/__init__.py |  |  | 0.813 |
| walker |  | 821 | 25 | python body src/flask/json/__init__.py:138 |  |  | 0.813 |
| walker |  | 845 | 24 | python names src/flask/wrappers.py |  |  | 0.813 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.771 |
| walker |  | 1021 | 176 | listing of 'docs' |  |  | 0.775 |
| walker |  | 1048 | 27 | listing of 'docs/_static' |  |  | 0.775 |
| walker |  | 1112 | 64 | listing of 'docs/deploying' |  |  | 0.775 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.707 |
| walker |  | 1190 | 78 | listing of 'docs/tutorial' |  |  | 0.707 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.637 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.621 |
| walker |  | 1530 | 340 | python names src/flask/__init__.py #1 |  |  | 0.750 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.733 |
| walker |  | 1573 | 43 | python body src/flask/json/__init__.py:77 |  |  | 0.733 |
| walker |  | 1616 | 43 | python body src/flask/json/__init__.py:108 |  |  | 0.733 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.707 |
| walker |  | 1672 | 56 | python body src/flask/json/__init__.py:13 |  |  | 0.707 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.684 |
| walker |  | 1794 | 122 | README.md section #0 |  |  | 0.725 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.694 |
| walker |  | 1939 | 145 | listing of 'docs/patterns' |  |  | 0.694 |
| walker |  | 1995 | 56 | python names src/flask/config.py |  |  | 0.694 |
| walker |  | 2056 | 61 | python names src/flask/views.py |  |  | 0.694 |
| walker |  | 2088 | 32 | python decl src/flask/views.py:11 |  |  | 0.694 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.660 |
| walker |  | 2136 | 48 | python decl src/flask/views.py:138 |  |  | 0.660 |
| walker |  | 2281 | 145 | listing of 'tests' |  |  | 0.662 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.646 |
| walker |  | 2341 | 60 | python body src/flask/json/__init__.py:47 |  |  | 0.646 |
| walker |  | 2407 | 66 | python decl src/flask/blueprints.py:18 |  |  | 0.646 |
| walker |  | 2492 | 85 | [package] in pyproject.toml |  |  | 0.655 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.629 |
| walker |  | 2530 | 38 | package metadata in pyproject.toml |  |  | 0.638 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.619 |
| walker |  | 2675 | 145 | [dependencies] in pyproject.toml |  |  | 0.673 |
| walker |  | 2718 | 43 | python names src/flask/json/provider.py |  |  | 0.674 |
| walker |  | 2737 | 19 | listing of 'tests/type_check' |  |  | 0.674 |
| walker |  | 2810 | 73 | python decl src/flask/wrappers.py:222 |  |  | 0.674 |
| walker |  | 2816 | 6 | python decl src/flask/wrappers.py:246 |  |  | 0.674 |
| walker |  | 2893 | 77 | python names src/flask/logging.py |  |  | 0.674 |
| walker |  | 2901 | 8 | python decl src/flask/logging.py:15 |  |  | 0.674 |
| walker |  | 2983 | 82 | python names src/flask/testing.py |  |  | 0.676 |
| walker |  | 3021 | 38 | python decl src/flask/testing.py:27 |  |  | 0.676 |
| walker |  | 3063 | 42 | python decl src/flask/testing.py:265 |  |  | 0.676 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.626 |
| walker |  | 3099 | 36 | python decl src/flask/testing.py:275 |  |  | 0.626 |
| walker |  | 3141 | 42 | python decl src/flask/blueprints.py:104 |  |  | 0.626 |
| walker |  | 3218 | 77 | README headline in src/flask/sansio/README.md |  |  | 0.644 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.625 |
| walker |  | 3467 | 249 | python names src/flask/cli.py |  |  | 0.628 |
| walker |  | 3493 | 26 | python decl src/flask/cli.py:293 |  |  | 0.628 |
| walker |  | 3529 | 36 | python decl src/flask/cli.py:241 |  |  | 0.628 |
| walker |  | 3577 | 48 | python decl src/flask/cli.py:235 |  |  | 0.628 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.602 |
| walker |  | 3624 | 47 | python decl src/flask/cli.py:229 |  |  | 0.602 |
| walker |  | 3685 | 61 | python decl src/flask/cli.py:283 |  |  | 0.602 |
| walker |  | 3709 | 24 | listing of 'examples/celery' |  |  | 0.602 |
| walker |  | 3713 | 4 | listing of 'examples/celery/src' |  |  | 0.602 |
| walker |  | 3730 | 17 | listing of 'examples/celery/src/task_app' |  |  | 0.602 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.588 |
| walker |  | 3864 | 134 | manifest config in pyproject.toml |  |  | 0.588 |
| walker |  | 3891 | 27 | listing of 'examples/javascript' |  |  | 0.588 |
| walker |  | 3904 | 13 | listing of 'examples/javascript/js_example' |  |  | 0.588 |
| walker |  | 3931 | 27 | listing of 'examples/tutorial' |  |  | 0.588 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.575 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.569 |
| walker |  | 4037 | 106 | python names src/flask/debughelpers.py |  |  | 0.577 |
| walker |  | 4057 | 20 | python decl src/flask/debughelpers.py:50 |  |  | 0.577 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.576 |
| walker |  | 4097 | 40 | python decl src/flask/debughelpers.py:23 |  |  | 0.576 |
| walker |  | 4104 | 7 | python body src/flask/debughelpers.py:46 |  |  | 0.576 |
| walker |  | 4198 | 94 | python decl src/flask/debughelpers.py:124 |  |  | 0.576 |
| walker |  | 4227 | 29 | listing of 'tests/test_apps' |  |  | 0.576 |
| walker |  | 4236 | 9 | listing of 'tests/test_apps/blueprintapp' |  |  | 0.576 |
| walker |  | 4245 | 9 | listing of 'tests/test_apps/subdomaintestmodule' |  |  | 0.576 |
| walker |  | 4259 | 14 | listing of 'tests/test_apps/blueprintapp/apps' |  |  | 0.576 |
| walker |  | 4268 | 9 | listing of 'tests/test_apps/blueprintapp/apps/frontend' |  |  | 0.576 |
| walker |  | 4280 | 12 | listing of 'tests/test_apps/blueprintapp/apps/admin' |  |  | 0.576 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.559 |
| walker |  | 4310 | 30 | listing of 'examples/tutorial/tests' |  |  | 0.559 |
| walker |  | 4432 | 122 | python names src/flask/sessions.py |  |  | 0.576 |
| walker |  | 4500 | 68 | python decl src/flask/sessions.py:83 |  |  | 0.576 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.560 |
| walker |  | 4514 | 14 | python doc src/flask/debughelpers.py:124 |  |  | 0.560 |
| walker |  | 4542 | 28 | listing of 'examples/tutorial/flaskr' |  |  | 0.560 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.546 |
| walker |  | 4667 | 125 | python decl src/flask/sessions.py:57 |  |  | 0.547 |
| walker |  | 4702 | 35 | python decl src/flask/sessions.py:73 |  |  | 0.547 |
| walker |  | 4832 | 130 | python decl src/flask/config.py:20 |  |  | 0.550 |
| walker |  | 4840 | 8 | python decl src/flask/config.py:32 |  |  | 0.550 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.536 |
| walker |  | 4850 | 10 | python decl src/flask/config.py:29 |  |  | 0.536 |
| walker |  | 4862 | 12 | python doc src/flask/config.py:20 |  |  | 0.536 |
| walker |  | 4899 | 37 | python decl src/flask/config.py:23 |  |  | 0.536 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.531 |
| walker |  | 5031 | 132 | python decl src/flask/testing.py:109 |  |  | 0.532 |
| walker |  | 5072 | 41 | python decl src/flask/testing.py:135 |  |  | 0.532 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.526 |
| walker |  | 5126 | 54 | python decl src/flask/testing.py:255 |  |  | 0.526 |
| walker |  | 5189 | 63 | python decl src/flask/testing.py:204 |  |  | 0.526 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.518 |
| walker |  | 5322 | 133 | python decl src/flask/config.py:50 |  |  | 0.542 |
| walker |  | 5353 | 31 | python decl src/flask/config.py:187 |  |  | 0.542 |
| walker |  | 5389 | 36 | python decl src/flask/config.py:304 |  |  | 0.542 |
| walker |  | 5426 | 37 | python decl src/flask/config.py:323 |  |  | 0.542 |
| walker |  | 5466 | 40 | python decl src/flask/config.py:126 |  |  | 0.542 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.530 |
| walker |  | 5515 | 49 | python decl src/flask/config.py:94 |  |  | 0.530 |
| walker |  | 5590 | 75 | README.md section #3 |  |  | 0.530 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.521 |
| walker |  | 5665 | 75 | python decl src/flask/config.py:256 |  |  | 0.521 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.526 |
| walker |  | 5815 | 150 | python names src/flask/ctx.py |  |  | 0.531 |
| walker |  | 5842 | 27 | python decl src/flask/ctx.py:118 |  |  | 0.531 |
| walker |  | 5858 | 16 | python body src/flask/ctx.py:235 |  |  | 0.531 |
| walker |  | 5881 | 23 | python body src/flask/ctx.py:209 |  |  | 0.531 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.525 |
| walker |  | 5971 | 90 | README.md section #2 |  |  | 0.525 |
| walker |  | 6048 | 77 | python decl src/flask/cli.py:305 |  |  | 0.525 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.518 |
| walker |  | 6203 | 155 | python names src/flask/templating.py |  |  | 0.522 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.515 |
| walker |  | 6229 | 26 | python decl src/flask/templating.py:36 |  |  | 0.515 |
| walker |  | 6267 | 38 | python decl src/flask/templating.py:136 |  |  | 0.515 |
| walker |  | 6307 | 40 | python decl src/flask/templating.py:181 |  |  | 0.515 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.509 |
| walker |  | 6412 | 105 | python decl src/flask/templating.py:49 |  |  | 0.509 |
| walker |  | 6452 | 40 | python decl src/flask/templating.py:57 |  |  | 0.509 |
| walker |  | 6484 | 32 | python decl src/flask/templating.py:163 |  |  | 0.509 |
| walker |  | 6519 | 35 | listing of 'tests/test_apps/cliapp' |  |  | 0.509 |
| walker |  | 6529 | 10 | listing of 'tests/test_apps/cliapp/inner1' |  |  | 0.509 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.500 |
| walker |  | 6541 | 12 | listing of 'tests/test_apps/cliapp/inner1/inner2' |  |  | 0.500 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.491 |
| walker |  | 6832 | 291 | python names src/flask/helpers.py |  |  | 0.494 |
| walker |  | 6839 | 7 | python decl src/flask/helpers.py:644 |  |  | 0.494 |
| walker |  | 6876 | 37 | python decl src/flask/helpers.py:51 |  |  | 0.494 |
| walker |  | 6913 | 37 | python decl src/flask/helpers.py:254 |  |  | 0.494 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.509 |
| walker |  | 6952 | 39 | python decl src/flask/helpers.py:360 |  |  | 0.509 |
| walker |  | 7000 | 48 | python decl src/flask/helpers.py:57 |  |  | 0.509 |
| walker |  | 7049 | 49 | python decl src/flask/helpers.py:543 |  |  | 0.509 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.504 |
| walker |  | 7104 | 55 | python decl src/flask/helpers.py:63 |  |  | 0.504 |
| walker |  | 7194 | 90 | python decl src/flask/helpers.py:200 |  |  | 0.504 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.495 |
| walker |  | 7337 | 143 | python decl src/flask/helpers.py:417 |  |  | 0.495 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.493 |
| walker |  | 7523 | 186 | python names src/flask/signals.py |  |  | 0.506 |
| walker |  | 7710 | 187 | python names src/flask/globals.py |  |  | 0.506 |
| walker |  | 7730 | 20 | python decl src/flask/globals.py:41 |  |  | 0.506 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.499 |
| walker |  | 7753 | 23 | python decl src/flask/globals.py:44 |  |  | 0.499 |
| walker |  | 7776 | 23 | python decl src/flask/globals.py:47 |  |  | 0.499 |
| walker |  | 7799 | 23 | python decl src/flask/globals.py:57 |  |  | 0.500 |
| walker |  | 7822 | 23 | python decl src/flask/globals.py:60 |  |  | 0.500 |
| walker |  | 7872 | 50 | python decl src/flask/globals.py:51 |  |  | 0.500 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.491 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.487 |
| walker |  | 8195 | 323 | python names src/flask/typing.py |  |  | 0.487 |
| walker |  | 8230 | 35 | python decl src/flask/typing.py:84 |  |  | 0.487 |
| walker |  | 8267 | 37 | python decl src/flask/typing.py:64 |  |  | 0.487 |
| walker |  | 8306 | 39 | python decl src/flask/typing.py:29 |  |  | 0.487 |
| walker |  | 8345 | 39 | python decl src/flask/typing.py:50 |  |  | 0.487 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.480 |
| walker |  | 8386 | 41 | python decl src/flask/typing.py:55 |  |  | 0.480 |
| walker |  | 8427 | 41 | python decl src/flask/typing.py:60 |  |  | 0.480 |
| walker |  | 8468 | 41 | python decl src/flask/typing.py:79 |  |  | 0.480 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.489 |
| walker |  | 8531 | 63 | python decl src/flask/typing.py:36 |  |  | 0.489 |
| walker |  | 8650 | 119 | python decl src/flask/typing.py:12 |  |  | 0.489 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.501 |
| walker |  | 8785 | 135 | README.md section #1 |  |  | 0.531 |
| walker |  | 8885 | 100 | python decl src/flask/testing.py:49 |  |  | 0.531 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.524 |
| walker |  | 9005 | 120 | python names src/flask/json/tag.py |  |  | 0.538 |
| walker |  | 9068 | 63 | python decl src/flask/json/tag.py:119 |  |  | 0.538 |
| walker |  | 9131 | 63 | python decl src/flask/json/tag.py:147 |  |  | 0.538 |
| walker |  | 9215 | 84 | python decl src/flask/json/tag.py:133 |  |  | 0.538 |
| walker |  | 9222 | 7 | python body src/flask/json/tag.py:143 |  |  | 0.538 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.531 |
| walker |  | 9306 | 84 | python decl src/flask/json/tag.py:159 |  |  | 0.531 |
| walker |  | 9390 | 84 | python decl src/flask/json/tag.py:191 |  |  | 0.531 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.551 |
| walker |  | 9474 | 84 | python decl src/flask/json/tag.py:205 |  |  | 0.551 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.565 |
| walker |  | 9560 | 86 | python decl src/flask/json/tag.py:93 |  |  | 0.565 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.573 |
| walker |  | 9646 | 86 | python decl src/flask/json/tag.py:173 |  |  | 0.573 |
| walker |  | 9853 | 207 | python names src/flask/app.py |  |  | 0.574 |
| walker |  | 9876 | 23 | python decl src/flask/app.py:64 |  |  | 0.574 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.560 |

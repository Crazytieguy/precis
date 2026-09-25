Score(3000)=0.676 I=0.885 C=0.516 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.771/0.637/0.715/0.676/0.569/0.509/0.534

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
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.684 |
| walker |  | 1763 | 147 | [dependencies] in pyproject.toml |  |  | 0.707 |
| walker |  | 1819 | 56 | python body src/flask/json/__init__.py:13 |  |  | 0.707 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.677 |
| walker |  | 1941 | 122 | README.md section #0 |  |  | 0.715 |
| walker |  | 2086 | 145 | listing of 'docs/patterns' |  |  | 0.715 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.680 |
| walker |  | 2142 | 56 | python names src/flask/config.py |  |  | 0.681 |
| walker |  | 2203 | 61 | python names src/flask/views.py |  |  | 0.681 |
| walker |  | 2235 | 32 | python decl src/flask/views.py:11 |  |  | 0.681 |
| walker |  | 2283 | 48 | python decl src/flask/views.py:138 |  |  | 0.681 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.663 |
| walker |  | 2428 | 145 | listing of 'tests' |  |  | 0.665 |
| walker |  | 2488 | 60 | python body src/flask/json/__init__.py:47 |  |  | 0.665 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.639 |
| walker |  | 2571 | 83 | [package] in pyproject.toml |  |  | 0.673 |
| walker |  | 2609 | 38 | package metadata in pyproject.toml |  |  | 0.693 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.672 |
| walker |  | 2675 | 66 | python decl src/flask/blueprints.py:18 |  |  | 0.673 |
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
| walker |  | 3757 | 27 | listing of 'examples/javascript' |  |  | 0.588 |
| walker |  | 3770 | 13 | listing of 'examples/javascript/js_example' |  |  | 0.588 |
| walker |  | 3797 | 27 | listing of 'examples/tutorial' |  |  | 0.588 |
| walker |  | 3903 | 106 | python names src/flask/debughelpers.py |  |  | 0.596 |
| walker |  | 3923 | 20 | python decl src/flask/debughelpers.py:50 |  |  | 0.596 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.582 |
| walker |  | 3963 | 40 | python decl src/flask/debughelpers.py:23 |  |  | 0.582 |
| walker |  | 3970 | 7 | python body src/flask/debughelpers.py:46 |  |  | 0.582 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.576 |
| walker |  | 4064 | 94 | python decl src/flask/debughelpers.py:124 |  |  | 0.576 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.568 |
| walker |  | 4093 | 29 | listing of 'tests/test_apps' |  |  | 0.568 |
| walker |  | 4102 | 9 | listing of 'tests/test_apps/blueprintapp' |  |  | 0.568 |
| walker |  | 4111 | 9 | listing of 'tests/test_apps/subdomaintestmodule' |  |  | 0.568 |
| walker |  | 4125 | 14 | listing of 'tests/test_apps/blueprintapp/apps' |  |  | 0.568 |
| walker |  | 4134 | 9 | listing of 'tests/test_apps/blueprintapp/apps/frontend' |  |  | 0.568 |
| walker |  | 4146 | 12 | listing of 'tests/test_apps/blueprintapp/apps/admin' |  |  | 0.568 |
| walker |  | 4176 | 30 | listing of 'examples/tutorial/tests' |  |  | 0.568 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.551 |
| walker |  | 4298 | 122 | python names src/flask/sessions.py |  |  | 0.569 |
| walker |  | 4366 | 68 | python decl src/flask/sessions.py:83 |  |  | 0.569 |
| walker |  | 4380 | 14 | python doc src/flask/debughelpers.py:124 |  |  | 0.569 |
| walker |  | 4408 | 28 | listing of 'examples/tutorial/flaskr' |  |  | 0.570 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.553 |
| walker |  | 4533 | 125 | python decl src/flask/sessions.py:57 |  |  | 0.553 |
| walker |  | 4568 | 35 | python decl src/flask/sessions.py:73 |  |  | 0.553 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.539 |
| walker |  | 4698 | 130 | python decl src/flask/config.py:20 |  |  | 0.543 |
| walker |  | 4706 | 8 | python decl src/flask/config.py:32 |  |  | 0.543 |
| walker |  | 4716 | 10 | python decl src/flask/config.py:29 |  |  | 0.543 |
| walker |  | 4728 | 12 | python doc src/flask/config.py:20 |  |  | 0.543 |
| walker |  | 4765 | 37 | python decl src/flask/config.py:23 |  |  | 0.543 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.529 |
| walker |  | 4897 | 132 | python decl src/flask/testing.py:109 |  |  | 0.530 |
| walker |  | 4938 | 41 | python decl src/flask/testing.py:135 |  |  | 0.530 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.525 |
| walker |  | 4992 | 54 | python decl src/flask/testing.py:255 |  |  | 0.525 |
| walker |  | 5055 | 63 | python decl src/flask/testing.py:204 |  |  | 0.525 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.519 |
| walker |  | 5188 | 133 | python decl src/flask/config.py:50 |  |  | 0.544 |
| walker |  | 5219 | 31 | python decl src/flask/config.py:187 |  |  | 0.544 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.536 |
| walker |  | 5255 | 36 | python decl src/flask/config.py:304 |  |  | 0.536 |
| walker |  | 5292 | 37 | python decl src/flask/config.py:323 |  |  | 0.536 |
| walker |  | 5332 | 40 | python decl src/flask/config.py:126 |  |  | 0.536 |
| walker |  | 5381 | 49 | python decl src/flask/config.py:94 |  |  | 0.536 |
| walker |  | 5456 | 75 | README.md section #3 |  |  | 0.536 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.524 |
| walker |  | 5531 | 75 | python decl src/flask/config.py:256 |  |  | 0.524 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.515 |
| walker |  | 5681 | 150 | python names src/flask/ctx.py |  |  | 0.521 |
| walker |  | 5708 | 27 | python decl src/flask/ctx.py:118 |  |  | 0.521 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.525 |
| walker |  | 5724 | 16 | python body src/flask/ctx.py:235 |  |  | 0.525 |
| walker |  | 5747 | 23 | python body src/flask/ctx.py:209 |  |  | 0.525 |
| walker |  | 5837 | 90 | README.md section #2 |  |  | 0.525 |
| walker |  | 5914 | 77 | python decl src/flask/cli.py:305 |  |  | 0.525 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.519 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.512 |
| walker |  | 6069 | 155 | python names src/flask/templating.py |  |  | 0.516 |
| walker |  | 6095 | 26 | python decl src/flask/templating.py:36 |  |  | 0.516 |
| walker |  | 6133 | 38 | python decl src/flask/templating.py:136 |  |  | 0.516 |
| walker |  | 6173 | 40 | python decl src/flask/templating.py:181 |  |  | 0.516 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.509 |
| walker |  | 6278 | 105 | python decl src/flask/templating.py:49 |  |  | 0.509 |
| walker |  | 6318 | 40 | python decl src/flask/templating.py:57 |  |  | 0.509 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.503 |
| walker |  | 6350 | 32 | python decl src/flask/templating.py:163 |  |  | 0.503 |
| walker |  | 6385 | 35 | listing of 'tests/test_apps/cliapp' |  |  | 0.503 |
| walker |  | 6395 | 10 | listing of 'tests/test_apps/cliapp/inner1' |  |  | 0.503 |
| walker |  | 6407 | 12 | listing of 'tests/test_apps/cliapp/inner1/inner2' |  |  | 0.503 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.495 |
| walker |  | 6698 | 291 | python names src/flask/helpers.py |  |  | 0.498 |
| walker |  | 6705 | 7 | python decl src/flask/helpers.py:644 |  |  | 0.498 |
| walker |  | 6742 | 37 | python decl src/flask/helpers.py:51 |  |  | 0.498 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.489 |
| walker |  | 6779 | 37 | python decl src/flask/helpers.py:254 |  |  | 0.489 |
| walker |  | 6818 | 39 | python decl src/flask/helpers.py:360 |  |  | 0.489 |
| walker |  | 6866 | 48 | python decl src/flask/helpers.py:57 |  |  | 0.489 |
| walker |  | 6915 | 49 | python decl src/flask/helpers.py:543 |  |  | 0.489 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.504 |
| walker |  | 6970 | 55 | python decl src/flask/helpers.py:63 |  |  | 0.504 |
| walker |  | 7060 | 90 | python decl src/flask/helpers.py:200 |  |  | 0.504 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.499 |
| walker |  | 7203 | 143 | python decl src/flask/helpers.py:417 |  |  | 0.499 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.490 |
| walker |  | 7389 | 186 | python names src/flask/signals.py |  |  | 0.504 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.501 |
| walker |  | 7576 | 187 | python names src/flask/globals.py |  |  | 0.502 |
| walker |  | 7596 | 20 | python decl src/flask/globals.py:41 |  |  | 0.502 |
| walker |  | 7619 | 23 | python decl src/flask/globals.py:44 |  |  | 0.502 |
| walker |  | 7642 | 23 | python decl src/flask/globals.py:47 |  |  | 0.502 |
| walker |  | 7665 | 23 | python decl src/flask/globals.py:57 |  |  | 0.502 |
| walker |  | 7688 | 23 | python decl src/flask/globals.py:60 |  |  | 0.502 |
| walker |  | 7738 | 50 | python decl src/flask/globals.py:51 |  |  | 0.502 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.495 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.486 |
| walker |  | 8061 | 323 | python names src/flask/typing.py |  |  | 0.486 |
| walker |  | 8096 | 35 | python decl src/flask/typing.py:84 |  |  | 0.486 |
| walker |  | 8133 | 37 | python decl src/flask/typing.py:64 |  |  | 0.486 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.482 |
| walker |  | 8172 | 39 | python decl src/flask/typing.py:29 |  |  | 0.482 |
| walker |  | 8211 | 39 | python decl src/flask/typing.py:50 |  |  | 0.482 |
| walker |  | 8252 | 41 | python decl src/flask/typing.py:55 |  |  | 0.482 |
| walker |  | 8293 | 41 | python decl src/flask/typing.py:60 |  |  | 0.482 |
| walker |  | 8334 | 41 | python decl src/flask/typing.py:79 |  |  | 0.482 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.476 |
| walker |  | 8397 | 63 | python decl src/flask/typing.py:36 |  |  | 0.476 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.485 |
| walker |  | 8516 | 119 | python decl src/flask/typing.py:12 |  |  | 0.485 |
| walker |  | 8651 | 135 | README.md section #1 |  |  | 0.515 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.526 |
| walker |  | 8751 | 100 | python decl src/flask/testing.py:49 |  |  | 0.526 |
| walker |  | 8871 | 120 | python names src/flask/json/tag.py |  |  | 0.541 |
| walker |  | 8934 | 63 | python decl src/flask/json/tag.py:119 |  |  | 0.541 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.534 |
| walker |  | 8997 | 63 | python decl src/flask/json/tag.py:147 |  |  | 0.534 |
| walker |  | 9081 | 84 | python decl src/flask/json/tag.py:133 |  |  | 0.534 |
| walker |  | 9088 | 7 | python body src/flask/json/tag.py:143 |  |  | 0.534 |
| walker |  | 9172 | 84 | python decl src/flask/json/tag.py:159 |  |  | 0.534 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.527 |
| walker |  | 9256 | 84 | python decl src/flask/json/tag.py:191 |  |  | 0.527 |
| walker |  | 9340 | 84 | python decl src/flask/json/tag.py:205 |  |  | 0.527 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.548 |
| walker |  | 9426 | 86 | python decl src/flask/json/tag.py:93 |  |  | 0.548 |
| walker |  | 9512 | 86 | python decl src/flask/json/tag.py:173 |  |  | 0.548 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.562 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.570 |
| walker |  | 9719 | 207 | python names src/flask/app.py |  |  | 0.571 |
| walker |  | 9742 | 23 | python decl src/flask/app.py:64 |  |  | 0.571 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.557 |

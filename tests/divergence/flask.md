Score(3000)=0.686 I=0.893 C=0.526 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.831/0.744/0.779/0.686/0.602/0.510/0.516

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
| walker |  | 374 | 13 | python imports in src/flask/__init__.py |  |  | 0.836 |
| walker |  | 386 | 12 | python imports #1 in src/flask/__init__.py |  |  | 0.837 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.763 |
| walker |  | 399 | 13 | python imports #2 in src/flask/__init__.py |  |  | 0.763 |
| walker |  | 411 | 12 | python imports #3 in src/flask/__init__.py |  |  | 0.764 |
| walker |  | 425 | 14 | listing of 'src/flask/json' |  |  | 0.825 |
| walker |  | 442 | 17 | listing of 'src/flask/sansio' |  |  | 0.916 |
| walker |  | 456 | 14 | python imports #7 in src/flask/__init__.py |  |  | 0.916 |
| walker |  | 522 | 66 | python imports #4 in src/flask/__init__.py |  |  | 0.922 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.789 |
| walker |  | 572 | 50 | python imports #5 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 598 | 26 | python imports #10 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 662 | 64 | python imports in src/flask/json/__init__.py |  |  | 0.795 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.810 |
| walker |  | 726 | 64 | python imports #9 in src/flask/__init__.py |  |  | 0.812 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.756 |
| walker |  | 870 | 144 | python imports #6 in src/flask/__init__.py |  |  | 0.831 |
| walker |  | 1046 | 176 | listing of 'docs' |  |  | 0.835 |
| walker |  | 1073 | 27 | listing of 'docs/_static' |  |  | 0.835 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.774 |
| walker |  | 1137 | 64 | listing of 'docs/deploying' |  |  | 0.774 |
| walker |  | 1215 | 78 | listing of 'docs/tutorial' |  |  | 0.774 |
| walker |  | 1227 | 12 | python imports in src/flask/__main__.py |  |  | 0.774 |
| walker |  | 1349 | 122 | README.md section #0 |  |  | 0.827 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.744 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.725 |
| walker |  | 1515 | 166 | python imports #8 in src/flask/__init__.py |  |  | 0.794 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.775 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.748 |
| walker |  | 1660 | 145 | listing of 'docs/patterns' |  |  | 0.748 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.724 |
| walker |  | 1805 | 145 | listing of 'tests' |  |  | 0.726 |
| walker |  | 1890 | 85 | [package] in pyproject.toml |  |  | 0.737 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.705 |
| walker |  | 1928 | 38 | package metadata in pyproject.toml |  |  | 0.716 |
| walker |  | 2073 | 145 | [dependencies] in pyproject.toml |  |  | 0.779 |
| walker |  | 2092 | 19 | listing of 'tests/type_check' |  |  | 0.779 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.741 |
| walker |  | 2215 | 123 | python decl names surface in src/flask/json/__init__.py |  |  | 0.741 |
| walker |  | 2215 | 0 | python decl at src/flask/json/__init__.py:13 |  |  | 0.741 |
| walker |  | 2215 | 0 | python decl at src/flask/json/__init__.py:47 |  |  | 0.741 |
| walker |  | 2215 | 0 | python decl at src/flask/json/__init__.py:77 |  |  | 0.741 |
| walker |  | 2215 | 0 | python decl at src/flask/json/__init__.py:108 |  |  | 0.741 |
| walker |  | 2215 | 0 | python decl at src/flask/json/__init__.py:138 |  |  | 0.741 |
| walker |  | 2226 | 11 | python decl doc at src/flask/json/__init__.py:13 |  |  | 0.741 |
| walker |  | 2237 | 11 | python decl doc at src/flask/json/__init__.py:77 |  |  | 0.741 |
| walker |  | 2252 | 15 | python decl doc at src/flask/json/__init__.py:108 |  |  | 0.741 |
| walker |  | 2268 | 16 | python decl doc at src/flask/json/__init__.py:47 |  |  | 0.741 |
| walker |  | 2293 | 25 | python decl body at src/flask/json/__init__.py:138 body 170 |  |  | 0.741 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.713 |
| walker |  | 2370 | 77 | README headline in src/flask/sansio/README.md |  |  | 0.734 |
| walker |  | 2394 | 24 | listing of 'examples/celery' |  |  | 0.734 |
| walker |  | 2398 | 4 | listing of 'examples/celery/src' |  |  | 0.734 |
| walker |  | 2415 | 17 | listing of 'examples/celery/src/task_app' |  |  | 0.735 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.705 |
| walker |  | 2549 | 134 | manifest config in pyproject.toml |  |  | 0.706 |
| walker |  | 2623 | 74 | python decl doc at src/flask/json/__init__.py:138 |  |  | 0.706 |
| walker |  | 2650 | 27 | listing of 'examples/javascript' |  |  | 0.706 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.685 |
| walker |  | 2663 | 13 | listing of 'examples/javascript/js_example' |  |  | 0.685 |
| walker |  | 2690 | 27 | listing of 'examples/tutorial' |  |  | 0.685 |
| walker |  | 2719 | 29 | listing of 'tests/test_apps' |  |  | 0.685 |
| walker |  | 2728 | 9 | listing of 'tests/test_apps/blueprintapp' |  |  | 0.685 |
| walker |  | 2737 | 9 | listing of 'tests/test_apps/subdomaintestmodule' |  |  | 0.685 |
| walker |  | 2751 | 14 | listing of 'tests/test_apps/blueprintapp/apps' |  |  | 0.685 |
| walker |  | 2760 | 9 | listing of 'tests/test_apps/blueprintapp/apps/frontend' |  |  | 0.685 |
| walker |  | 2772 | 12 | listing of 'tests/test_apps/blueprintapp/apps/admin' |  |  | 0.685 |
| walker |  | 2802 | 30 | listing of 'examples/tutorial/tests' |  |  | 0.685 |
| walker |  | 2830 | 28 | listing of 'examples/tutorial/flaskr' |  |  | 0.686 |
| walker |  | 2905 | 75 | README.md section #3 |  |  | 0.686 |
| walker |  | 2995 | 90 | README.md section #2 |  |  | 0.686 |
| walker |  | 3030 | 35 | listing of 'tests/test_apps/cliapp' |  |  | 0.686 |
| walker |  | 3040 | 10 | listing of 'tests/test_apps/cliapp/inner1' |  |  | 0.686 |
| walker |  | 3052 | 12 | listing of 'tests/test_apps/cliapp/inner1/inner2' |  |  | 0.686 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.635 |
| walker |  | 3187 | 135 | README.md section #1 |  |  | 0.693 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.673 |
| walker |  | 3513 | 326 | tool.flit+uv+pytest+coverage config in pyproject.toml |  |  | 0.673 |
| walker |  | 3524 | 11 | python imports in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.673 |
| walker |  | 3535 | 11 | python imports in tests/test_apps/subdomaintestmodule/__init__.py |  |  | 0.673 |
| walker |  | 3558 | 23 | tool.codespell config in pyproject.toml |  |  | 0.673 |
| walker |  | 3586 | 28 | python imports in src/flask/signals.py |  |  | 0.674 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.647 |
| walker |  | 3665 | 79 | python decl names surface in src/flask/cli.py |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:37 |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:293 |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:405 |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:531 |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:780 |  |  | 0.652 |
| walker |  | 3665 | 0 | python decl at src/flask/cli.py:867 |  |  | 0.652 |
| walker |  | 3681 | 16 | python decl doc at src/flask/cli.py:37 |  |  | 0.652 |
| walker |  | 3693 | 12 | python class body at src/flask/cli.py:780 |  |  | 0.652 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.636 |
| walker |  | 3757 | 64 | python decl doc at src/flask/cli.py:780 |  |  | 0.636 |
| walker |  | 3821 | 64 | python decl doc at src/flask/cli.py:867 |  |  | 0.636 |
| walker |  | 3829 | 8 | python decl names surface in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.636 |
| walker |  | 3889 | 60 | python decl body at src/flask/json/__init__.py:47 body 70 |  |  | 0.636 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.621 |
| walker |  | 3977 | 88 | python decl doc at src/flask/cli.py:405 |  |  | 0.621 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.615 |
| walker |  | 4001 | 24 | python decl names surface in src/flask/wrappers.py |  |  | 0.615 |
| walker |  | 4001 | 0 | python decl at src/flask/wrappers.py:18 |  |  | 0.615 |
| walker |  | 4001 | 0 | python decl at src/flask/wrappers.py:222 |  |  | 0.615 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.621 |
| walker |  | 4235 | 234 | python method sigs in src/flask/wrappers.py |  |  | 0.621 |
| walker |  | 4235 | 0 | python method at src/flask/wrappers.py:197 |  |  | 0.621 |
| walker |  | 4235 | 0 | python method at src/flask/wrappers.py:212 |  |  | 0.621 |
| walker |  | 4243 | 8 | python method at src/flask/wrappers.py:59 |  |  | 0.621 |
| walker |  | 4251 | 8 | python method at src/flask/wrappers.py:92 |  |  | 0.621 |
| walker |  | 4259 | 8 | python method at src/flask/wrappers.py:146 |  |  | 0.621 |
| walker |  | 4267 | 8 | python method at src/flask/wrappers.py:161 |  |  | 0.621 |
| walker |  | 4275 | 8 | python method at src/flask/wrappers.py:180 |  |  | 0.621 |
| walker |  | 4283 | 8 | python method at src/flask/wrappers.py:246 |  |  | 0.621 |
| walker |  | 4294 | 11 | python method at src/flask/wrappers.py:88 |  |  | 0.621 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.602 |
| walker |  | 4305 | 11 | python method at src/flask/wrappers.py:142 |  |  | 0.602 |
| walker |  | 4317 | 12 | python method at src/flask/wrappers.py:115 |  |  | 0.602 |
| walker |  | 4332 | 15 | python method at src/flask/wrappers.py:119 |  |  | 0.602 |
| walker |  | 4381 | 49 | python class body at src/flask/wrappers.py:222 |  |  | 0.602 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.585 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.573 |
| walker |  | 4785 | 404 | python class body at src/flask/wrappers.py:18 |  |  | 0.574 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.560 |
| walker |  | 4877 | 92 | python decl doc at src/flask/cli.py:531 |  |  | 0.560 |
| walker |  | 4916 | 39 | python imports in src/flask/typing.py |  |  | 0.560 |
| walker |  | 4931 | 15 | python decl names surface in src/flask/blueprints.py |  |  | 0.560 |
| walker |  | 4931 | 0 | python decl at src/flask/blueprints.py:18 |  |  | 0.560 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.556 |
| walker |  | 4997 | 66 | python method sigs in src/flask/blueprints.py |  |  | 0.556 |
| walker |  | 4997 | 0 | python method at src/flask/blueprints.py:55 |  |  | 0.556 |
| walker |  | 4997 | 0 | python method at src/flask/blueprints.py:82 |  |  | 0.556 |
| walker |  | 5039 | 42 | python method at src/flask/blueprints.py:104 |  |  | 0.556 |
| walker |  | 5060 | 21 | python imports in tests/test_apps/blueprintapp/apps/admin/__init__.py |  |  | 0.556 |
| walker |  | 5081 | 21 | python imports in tests/test_apps/blueprintapp/apps/frontend/__init__.py |  |  | 0.556 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.549 |
| walker |  | 5130 | 49 | declaration surface of docs/reqcontext.rst |  |  | 0.549 |
| walker |  | 5153 | 23 | python imports in examples/tutorial/flaskr/__init__.py |  |  | 0.549 |
| walker |  | 5164 | 11 | python decl names surface in examples/tutorial/flaskr/__init__.py |  |  | 0.549 |
| walker |  | 5164 | 0 | python decl at examples/tutorial/flaskr/__init__.py:6 |  |  | 0.549 |
| walker |  | 5225 | 61 | python decl names surface in src/flask/views.py |  |  | 0.550 |
| walker |  | 5225 | 0 | python decl at src/flask/views.py:16 |  |  | 0.550 |
| walker |  | 5225 | 0 | python decl at src/flask/views.py:138 |  |  | 0.550 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.542 |
| walker |  | 5257 | 32 | python decl at src/flask/views.py:11 |  |  | 0.542 |
| walker |  | 5307 | 50 | python decl doc at src/flask/views.py:138 |  |  | 0.542 |
| walker |  | 5384 | 77 | python method sigs in src/flask/views.py |  |  | 0.542 |
| walker |  | 5384 | 0 | python method at src/flask/views.py:78 |  |  | 0.542 |
| walker |  | 5384 | 0 | python method at src/flask/views.py:165 |  |  | 0.542 |
| walker |  | 5384 | 0 | python method at src/flask/views.py:182 |  |  | 0.542 |
| walker |  | 5396 | 12 | python method body at src/flask/views.py:78 body 83 |  |  | 0.542 |
| walker |  | 5438 | 42 | python method at src/flask/views.py:85 |  |  | 0.542 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.529 |
| walker |  | 5523 | 85 | python decl doc at src/flask/views.py:16 |  |  | 0.529 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.521 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.525 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.519 |
| walker |  | 5974 | 451 | python class body at src/flask/views.py:16 |  |  | 0.519 |
| walker |  | 6025 | 51 | python method doc at src/flask/views.py:78 |  |  | 0.519 |
| walker |  | 6039 | 14 | python decl body at src/flask/json/__init__.py:77 body 105 |  |  | 0.519 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.512 |
| walker |  | 6102 | 63 | python method doc at src/flask/wrappers.py:246 |  |  | 0.512 |
| walker |  | 6184 | 82 | python decl names surface in src/flask/testing.py |  |  | 0.517 |
| walker |  | 6184 | 0 | python decl at src/flask/testing.py:27 |  |  | 0.517 |
| walker |  | 6184 | 0 | python decl at src/flask/testing.py:100 |  |  | 0.517 |
| walker |  | 6184 | 0 | python decl at src/flask/testing.py:109 |  |  | 0.517 |
| walker |  | 6184 | 0 | python decl at src/flask/testing.py:265 |  |  | 0.517 |
| walker |  | 6195 | 11 | python class body at src/flask/testing.py:109 |  |  | 0.517 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.510 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.504 |
| walker |  | 6391 | 196 | python method sigs in src/flask/testing.py |  |  | 0.505 |
| walker |  | 6391 | 0 | python method at src/flask/testing.py:88 |  |  | 0.505 |
| walker |  | 6391 | 0 | python method at src/flask/testing.py:125 |  |  | 0.505 |
| walker |  | 6391 | 0 | python method at src/flask/testing.py:185 |  |  | 0.505 |
| walker |  | 6391 | 0 | python method at src/flask/testing.py:249 |  |  | 0.505 |
| walker |  | 6391 | 0 | python method at src/flask/testing.py:271 |  |  | 0.505 |
| walker |  | 6427 | 36 | python method at src/flask/testing.py:275 |  |  | 0.505 |
| walker |  | 6468 | 41 | python method at src/flask/testing.py:135 |  |  | 0.505 |
| walker |  | 6522 | 54 | python method at src/flask/testing.py:255 |  |  | 0.505 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.497 |
| walker |  | 6586 | 64 | python decl doc at src/flask/testing.py:265 |  |  | 0.497 |
| walker |  | 6649 | 63 | python method at src/flask/testing.py:204 |  |  | 0.497 |
| walker |  | 6682 | 33 | python method at src/flask/testing.py:193 |  |  | 0.497 |
| walker |  | 6741 | 59 | python method doc at src/flask/testing.py:88 |  |  | 0.497 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.487 |
| walker |  | 6841 | 100 | python method at src/flask/testing.py:49 |  |  | 0.487 |
| walker |  | 6896 | 55 | python imports in src/flask/globals.py |  |  | 0.487 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.479 |
| walker |  | 6966 | 70 | python decl names surface in src/flask/logging.py |  |  | 0.479 |
| walker |  | 6966 | 0 | python decl at src/flask/logging.py:31 |  |  | 0.479 |
| walker |  | 6966 | 0 | python decl at src/flask/logging.py:58 |  |  | 0.479 |
| walker |  | 6981 | 15 | python decl at src/flask/logging.py:15 |  |  | 0.479 |
| walker |  | 7030 | 49 | python decl doc at src/flask/logging.py:31 |  |  | 0.479 |
| walker |  | 7039 | 9 | python decl body at src/flask/logging.py:15 body 28 |  |  | 0.479 |
| walker |  | 7052 | 13 | listing of 'tests/static' |  |  | 0.479 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.474 |
| walker |  | 7066 | 14 | python decl names surface in examples/javascript/js_example/__init__.py |  |  | 0.474 |
| walker |  | 7090 | 24 | python imports in examples/javascript/js_example/__init__.py |  |  | 0.474 |
| walker |  | 7104 | 14 | python decl names surface in tests/test_apps/blueprintapp/__init__.py |  |  | 0.474 |
| walker |  | 7254 | 150 | python decl names surface in src/flask/ctx.py |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:30 |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:154 |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:209 |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:235 |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:260 |  |  | 0.476 |
| walker |  | 7254 | 0 | python decl at src/flask/ctx.py:528 |  |  | 0.476 |
| walker |  | 7281 | 27 | python decl at src/flask/ctx.py:118 |  |  | 0.476 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.467 |
| walker |  | 7301 | 20 | python decl doc at src/flask/ctx.py:209 |  |  | 0.467 |
| walker |  | 7317 | 16 | python decl body at src/flask/ctx.py:235 body 257 |  |  | 0.467 |
| walker |  | 7340 | 23 | python decl body at src/flask/ctx.py:209 body 232 |  |  | 0.467 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.480 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.496 |
| walker |  | 7762 | 422 | python method sigs in src/flask/ctx.py |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:53 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:59 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:62 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:68 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:79 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:93 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:105 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:108 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:111 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:355 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:381 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:405 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:416 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:446 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:506 |  |  | 0.497 |
| walker |  | 7762 | 0 | python method at src/flask/ctx.py:518 |  |  | 0.497 |
| walker |  | 7770 | 8 | python method at src/flask/ctx.py:339 |  |  | 0.497 |
| walker |  | 7778 | 8 | python method at src/flask/ctx.py:350 |  |  | 0.497 |
| walker |  | 7786 | 8 | python method at src/flask/ctx.py:370 |  |  | 0.497 |
| walker |  | 7794 | 8 | python method at src/flask/ctx.py:395 |  |  | 0.497 |
| walker |  | 7810 | 16 | python method doc at src/flask/ctx.py:350 |  |  | 0.497 |
| walker |  | 7868 | 58 | python method at src/flask/ctx.py:300 |  |  | 0.497 |
| walker |  | 7926 | 58 | python method at src/flask/ctx.py:510 |  |  | 0.497 |
| walker |  | 7983 | 57 | python decl doc at src/flask/ctx.py:154 |  |  | 0.497 |
| walker |  | 8021 | 38 | python method doc at src/flask/ctx.py:405 |  |  | 0.497 |
| walker |  | 8041 | 20 | python method doc at src/flask/ctx.py:381 |  |  | 0.487 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.487 |
| walker |  | 8134 | 93 | python decl doc at src/flask/ctx.py:260 |  |  | 0.487 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.484 |
| walker |  | 8189 | 55 | python method doc at src/flask/ctx.py:370 |  |  | 0.484 |
| walker |  | 8252 | 63 | python method doc at src/flask/ctx.py:339 |  |  | 0.484 |
| walker |  | 8320 | 68 | python method doc at src/flask/ctx.py:395 |  |  | 0.484 |
| walker |  | 8324 | 4 | listing of 'examples/tutorial/flaskr/static' |  |  | 0.484 |
| walker |  | 8328 | 4 | listing of 'tests/test_apps/subdomaintestmodule/static' |  |  | 0.484 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.477 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.487 |
| walker |  | 8586 | 258 | python decl names surface in src/flask/helpers.py |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:28 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:36 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:151 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:281 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:304 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:326 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:402 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:587 |  |  | 0.501 |
| walker |  | 8586 | 0 | python decl at src/flask/helpers.py:654 |  |  | 0.501 |
| walker |  | 8623 | 37 | python decl at src/flask/helpers.py:254 |  |  | 0.501 |
| walker |  | 8662 | 39 | python decl at src/flask/helpers.py:360 |  |  | 0.501 |
| walker |  | 8680 | 18 | python decl at src/flask/helpers.py:644 |  |  | 0.503 |
| walker |  | 8728 | 48 | python decl at src/flask/helpers.py:51 |  |  | 0.506 |
| walker |  | 8728 | 0 | python decl body at src/flask/helpers.py:51 body 54 |  |  | 0.506 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.500 |
| walker |  | 8777 | 49 | python decl at src/flask/helpers.py:543 |  |  | 0.500 |
| walker |  | 8797 | 20 | python decl doc at src/flask/helpers.py:543 |  |  | 0.500 |
| walker |  | 8852 | 55 | python decl at src/flask/helpers.py:63 |  |  | 0.500 |
| walker |  | 8911 | 59 | python decl at src/flask/helpers.py:57 |  |  | 0.502 |
| walker |  | 8911 | 0 | python decl body at src/flask/helpers.py:57 body 60 |  |  | 0.502 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.516 |
| walker |  | 8973 | 62 | python method sigs in src/flask/helpers.py |  |  | 0.516 |
| walker |  | 8973 | 0 | python method at src/flask/helpers.py:659 |  |  | 0.516 |
| walker |  | 8973 | 0 | python method at src/flask/helpers.py:662 |  |  | 0.516 |
| walker |  | 8973 | 0 | python method at src/flask/helpers.py:676 |  |  | 0.516 |
| walker |  | 8986 | 13 | python method doc at src/flask/helpers.py:676 |  |  | 0.516 |
| walker |  | 9036 | 50 | python decl doc at src/flask/helpers.py:28 |  |  | 0.516 |
| walker |  | 9126 | 90 | python decl at src/flask/helpers.py:200 |  |  | 0.516 |
| walker |  | 9144 | 18 | python decl doc at src/flask/helpers.py:200 |  |  | 0.516 |
| walker |  | 9203 | 59 | python method at src/flask/helpers.py:665 |  |  | 0.516 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.510 |
| walker |  | 9280 | 77 | python decl doc at src/flask/helpers.py:63 |  |  | 0.510 |
| walker |  | 9322 | 42 | python decl doc at src/flask/helpers.py:654 |  |  | 0.510 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.531 |
| walker |  | 9404 | 82 | python decl doc at src/flask/helpers.py:36 |  |  | 0.531 |
| walker |  | 9495 | 91 | python decl doc at src/flask/helpers.py:587 |  |  | 0.531 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.546 |
| walker |  | 9593 | 98 | python decl doc at src/flask/helpers.py:360 |  |  | 0.546 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.554 |
| walker |  | 9736 | 143 | python decl at src/flask/helpers.py:417 |  |  | 0.554 |
| walker |  | 9752 | 16 | python decl doc at src/flask/helpers.py:417 |  |  | 0.554 |
| walker |  | 9855 | 103 | python decl doc at src/flask/helpers.py:151 |  |  | 0.554 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.541 |

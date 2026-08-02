Score(3000)=0.686 I=0.893 C=0.526 ns_rows≤3K=18/57 (reached=9 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | listing of '.' |  |  | 0.000 |
| walker |  | 59 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 86 | 27 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| walker |  | 96 | 10 | listing of '.devcontainer' |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| walker |  | 107 | 11 | listing of 'examples' |  |  | 0.000 |
| walker |  | 121 | 14 | listing of '.github' |  |  | 0.000 |
| walker |  | 143 | 22 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 153 |  | 56 | Repository root listing (complete) | 1.2 |  | 0.488 |
| walker |  | 245 | 102 | README headline in README.md |  |  | 1.000 |
| walker |  | 275 | 30 | headings outline in README.md |  |  | 1.000 |
| ns | 277 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.592 |
| walker |  | 367 | 92 | listing of 'src/flask' |  |  | 0.836 |
| walker |  | 380 | 13 | python imports in src/flask/__init__.py |  |  | 0.836 |
| walker |  | 392 | 12 | python imports #1 in src/flask/__init__.py |  |  | 0.837 |
| ns | 401 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.763 |
| walker |  | 405 | 13 | python imports #2 in src/flask/__init__.py |  |  | 0.763 |
| walker |  | 417 | 12 | python imports #3 in src/flask/__init__.py |  |  | 0.764 |
| walker |  | 430 | 13 | listing of 'src/flask/json' |  |  | 0.825 |
| walker |  | 446 | 16 | listing of 'src/flask/sansio' |  |  | 0.916 |
| walker |  | 460 | 14 | python imports #7 in src/flask/__init__.py |  |  | 0.916 |
| walker |  | 526 | 66 | python imports #4 in src/flask/__init__.py |  |  | 0.922 |
| ns | 547 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.789 |
| walker |  | 576 | 50 | python imports #5 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 602 | 26 | python imports #10 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 666 | 64 | python imports in src/flask/json/__init__.py |  |  | 0.795 |
| ns | 713 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.810 |
| walker |  | 730 | 64 | python imports #9 in src/flask/__init__.py |  |  | 0.812 |
| ns | 871 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.756 |
| walker |  | 874 | 144 | python imports #6 in src/flask/__init__.py |  |  | 0.831 |
| walker |  | 1053 | 179 | listing of 'docs' |  |  | 0.835 |
| walker |  | 1079 | 26 | listing of 'docs/_static' |  |  | 0.835 |
| ns | 1127 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.774 |
| walker |  | 1142 | 63 | listing of 'docs/deploying' |  |  | 0.774 |
| walker |  | 1219 | 77 | listing of 'docs/tutorial' |  |  | 0.774 |
| walker |  | 1231 | 12 | python imports in src/flask/__main__.py |  |  | 0.774 |
| walker |  | 1353 | 122 | README.md section #0 |  |  | 0.827 |
| ns | 1397 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.744 |
| ns | 1474 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.725 |
| walker |  | 1519 | 166 | python imports #8 in src/flask/__init__.py |  |  | 0.794 |
| ns | 1551 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.775 |
| ns | 1645 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.748 |
| walker |  | 1663 | 144 | listing of 'docs/patterns' |  |  | 0.748 |
| ns | 1751 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.724 |
| walker |  | 1811 | 148 | listing of 'tests' |  |  | 0.726 |
| walker |  | 1896 | 85 | [package] in pyproject.toml |  |  | 0.737 |
| ns | 1902 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.705 |
| walker |  | 1934 | 38 | package metadata in pyproject.toml |  |  | 0.716 |
| walker |  | 1952 | 18 | listing of 'tests/type_check' |  |  | 0.716 |
| walker |  | 2097 | 145 | [dependencies] in pyproject.toml |  |  | 0.779 |
| ns | 2118 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.741 |
| walker |  | 2220 | 123 | python decl names surface in src/flask/json/__init__.py |  |  | 0.741 |
| walker |  | 2220 | 0 | python decl at src/flask/json/__init__.py:13 |  |  | 0.741 |
| walker |  | 2220 | 0 | python decl at src/flask/json/__init__.py:47 |  |  | 0.741 |
| walker |  | 2220 | 0 | python decl at src/flask/json/__init__.py:77 |  |  | 0.741 |
| walker |  | 2220 | 0 | python decl at src/flask/json/__init__.py:108 |  |  | 0.741 |
| walker |  | 2220 | 0 | python decl at src/flask/json/__init__.py:138 |  |  | 0.741 |
| walker |  | 2231 | 11 | python decl doc at src/flask/json/__init__.py:13 |  |  | 0.741 |
| walker |  | 2242 | 11 | python decl doc at src/flask/json/__init__.py:77 |  |  | 0.741 |
| walker |  | 2257 | 15 | python decl doc at src/flask/json/__init__.py:108 |  |  | 0.741 |
| walker |  | 2273 | 16 | python decl doc at src/flask/json/__init__.py:47 |  |  | 0.741 |
| walker |  | 2298 | 25 | python decl body at src/flask/json/__init__.py:138 body 170 |  |  | 0.741 |
| ns | 2300 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.713 |
| walker |  | 2375 | 77 | README headline in src/flask/sansio/README.md |  |  | 0.734 |
| walker |  | 2399 | 24 | listing of 'examples/celery' |  |  | 0.734 |
| walker |  | 2403 | 4 | listing of 'examples/celery/src' |  |  | 0.734 |
| walker |  | 2420 | 17 | listing of 'examples/celery/src/task_app' |  |  | 0.735 |
| ns | 2516 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.705 |
| walker |  | 2554 | 134 | manifest config in pyproject.toml |  |  | 0.706 |
| walker |  | 2628 | 74 | python decl doc at src/flask/json/__init__.py:138 |  |  | 0.706 |
| walker |  | 2656 | 28 | listing of 'examples/javascript' |  |  | 0.706 |
| ns | 2661 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.685 |
| walker |  | 2669 | 13 | listing of 'examples/javascript/js_example' |  |  | 0.685 |
| walker |  | 2697 | 28 | listing of 'examples/tutorial' |  |  | 0.685 |
| walker |  | 2726 | 29 | listing of 'examples/tutorial/tests' |  |  | 0.685 |
| walker |  | 2758 | 32 | listing of 'tests/test_apps' |  |  | 0.685 |
| walker |  | 2767 | 9 | listing of 'tests/test_apps/blueprintapp' |  |  | 0.685 |
| walker |  | 2776 | 9 | listing of 'tests/test_apps/subdomaintestmodule' |  |  | 0.685 |
| walker |  | 2789 | 13 | listing of 'tests/test_apps/blueprintapp/apps' |  |  | 0.685 |
| walker |  | 2798 | 9 | listing of 'tests/test_apps/blueprintapp/apps/frontend' |  |  | 0.685 |
| walker |  | 2811 | 13 | listing of 'tests/test_apps/blueprintapp/apps/admin' |  |  | 0.685 |
| walker |  | 2840 | 29 | listing of 'examples/tutorial/flaskr' |  |  | 0.686 |
| walker |  | 2915 | 75 | README.md section #3 |  |  | 0.686 |
| walker |  | 3005 | 90 | README.md section #2 |  |  | 0.686 |
| walker |  | 3038 | 33 | listing of 'tests/test_apps/cliapp' |  |  | 0.686 |
| walker |  | 3048 | 10 | listing of 'tests/test_apps/cliapp/inner1' |  |  | 0.686 |
| walker |  | 3057 | 9 | listing of 'tests/test_apps/cliapp/inner1/inner2' |  |  | 0.686 |
| ns | 3091 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.635 |
| walker |  | 3192 | 135 | README.md section #1 |  |  | 0.693 |
| ns | 3252 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.673 |
| walker |  | 3518 | 326 | tool.flit+uv+pytest+coverage config in pyproject.toml |  |  | 0.673 |
| walker |  | 3529 | 11 | python imports in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.673 |
| walker |  | 3540 | 11 | python imports in tests/test_apps/subdomaintestmodule/__init__.py |  |  | 0.673 |
| walker |  | 3563 | 23 | tool.codespell config in pyproject.toml |  |  | 0.673 |
| walker |  | 3591 | 28 | python imports in src/flask/signals.py |  |  | 0.674 |
| ns | 3594 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.647 |
| walker |  | 3670 | 79 | python decl names surface in src/flask/cli.py |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:37 |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:293 |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:405 |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:531 |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:780 |  |  | 0.652 |
| walker |  | 3670 | 0 | python decl at src/flask/cli.py:867 |  |  | 0.652 |
| walker |  | 3686 | 16 | python decl doc at src/flask/cli.py:37 |  |  | 0.652 |
| walker |  | 3698 | 12 | python class body at src/flask/cli.py:780 |  |  | 0.652 |
| walker |  | 3762 | 64 | python decl doc at src/flask/cli.py:780 |  |  | 0.636 |
| ns | 3762 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.636 |
| walker |  | 3826 | 64 | python decl doc at src/flask/cli.py:867 |  |  | 0.636 |
| walker |  | 3834 | 8 | python decl names surface in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.636 |
| walker |  | 3894 | 60 | python decl body at src/flask/json/__init__.py:47 body 70 |  |  | 0.636 |
| ns | 3947 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.621 |
| walker |  | 3982 | 88 | python decl doc at src/flask/cli.py:405 |  |  | 0.621 |
| ns | 3999 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.615 |
| walker |  | 4006 | 24 | python decl names surface in src/flask/wrappers.py |  |  | 0.615 |
| walker |  | 4006 | 0 | python decl at src/flask/wrappers.py:18 |  |  | 0.615 |
| walker |  | 4006 | 0 | python decl at src/flask/wrappers.py:222 |  |  | 0.615 |
| ns | 4094 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.621 |
| walker |  | 4240 | 234 | python method sigs in src/flask/wrappers.py |  |  | 0.621 |
| walker |  | 4240 | 0 | python method at src/flask/wrappers.py:197 |  |  | 0.621 |
| walker |  | 4240 | 0 | python method at src/flask/wrappers.py:212 |  |  | 0.621 |
| walker |  | 4248 | 8 | python method at src/flask/wrappers.py:59 |  |  | 0.621 |
| walker |  | 4256 | 8 | python method at src/flask/wrappers.py:92 |  |  | 0.621 |
| walker |  | 4264 | 8 | python method at src/flask/wrappers.py:146 |  |  | 0.621 |
| walker |  | 4272 | 8 | python method at src/flask/wrappers.py:161 |  |  | 0.621 |
| walker |  | 4280 | 8 | python method at src/flask/wrappers.py:180 |  |  | 0.621 |
| walker |  | 4288 | 8 | python method at src/flask/wrappers.py:246 |  |  | 0.621 |
| walker |  | 4299 | 11 | python method at src/flask/wrappers.py:88 |  |  | 0.621 |
| ns | 4303 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.602 |
| walker |  | 4310 | 11 | python method at src/flask/wrappers.py:142 |  |  | 0.602 |
| walker |  | 4322 | 12 | python method at src/flask/wrappers.py:115 |  |  | 0.602 |
| walker |  | 4337 | 15 | python method at src/flask/wrappers.py:119 |  |  | 0.602 |
| walker |  | 4386 | 49 | python class body at src/flask/wrappers.py:222 |  |  | 0.602 |
| ns | 4515 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.585 |
| ns | 4670 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.573 |
| walker |  | 4790 | 404 | python class body at src/flask/wrappers.py:18 |  |  | 0.574 |
| ns | 4854 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.560 |
| walker |  | 4882 | 92 | python decl doc at src/flask/cli.py:531 |  |  | 0.560 |
| walker |  | 4921 | 39 | python imports in src/flask/typing.py |  |  | 0.560 |
| walker |  | 4936 | 15 | python decl names surface in src/flask/blueprints.py |  |  | 0.560 |
| walker |  | 4936 | 0 | python decl at src/flask/blueprints.py:18 |  |  | 0.560 |
| ns | 4955 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.556 |
| walker |  | 5002 | 66 | python method sigs in src/flask/blueprints.py |  |  | 0.556 |
| walker |  | 5002 | 0 | python method at src/flask/blueprints.py:55 |  |  | 0.556 |
| walker |  | 5002 | 0 | python method at src/flask/blueprints.py:82 |  |  | 0.556 |
| walker |  | 5044 | 42 | python method at src/flask/blueprints.py:104 |  |  | 0.556 |
| walker |  | 5065 | 21 | python imports in tests/test_apps/blueprintapp/apps/admin/__init__.py |  |  | 0.556 |
| walker |  | 5086 | 21 | python imports in tests/test_apps/blueprintapp/apps/frontend/__init__.py |  |  | 0.556 |
| walker |  | 5089 | 3 | listing of 'examples/tutorial/flaskr/static' |  |  | 0.556 |
| walker |  | 5092 | 3 | listing of 'tests/test_apps/subdomaintestmodule/static' |  |  | 0.556 |
| ns | 5103 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.549 |
| walker |  | 5141 | 49 | declaration surface of docs/reqcontext.rst |  |  | 0.549 |
| walker |  | 5164 | 23 | python imports in examples/tutorial/flaskr/__init__.py |  |  | 0.549 |
| walker |  | 5175 | 11 | python decl names surface in examples/tutorial/flaskr/__init__.py |  |  | 0.549 |
| walker |  | 5175 | 0 | python decl at examples/tutorial/flaskr/__init__.py:6 |  |  | 0.549 |
| walker |  | 5236 | 61 | python decl names surface in src/flask/views.py |  |  | 0.550 |
| walker |  | 5236 | 0 | python decl at src/flask/views.py:16 |  |  | 0.550 |
| walker |  | 5236 | 0 | python decl at src/flask/views.py:138 |  |  | 0.550 |
| ns | 5247 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.542 |
| walker |  | 5268 | 32 | python decl at src/flask/views.py:11 |  |  | 0.542 |
| walker |  | 5318 | 50 | python decl doc at src/flask/views.py:138 |  |  | 0.542 |
| walker |  | 5395 | 77 | python method sigs in src/flask/views.py |  |  | 0.542 |
| walker |  | 5395 | 0 | python method at src/flask/views.py:78 |  |  | 0.542 |
| walker |  | 5395 | 0 | python method at src/flask/views.py:165 |  |  | 0.542 |
| walker |  | 5395 | 0 | python method at src/flask/views.py:182 |  |  | 0.542 |
| walker |  | 5407 | 12 | python method body at src/flask/views.py:78 body 83 |  |  | 0.542 |
| walker |  | 5449 | 42 | python method at src/flask/views.py:85 |  |  | 0.542 |
| ns | 5493 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.529 |
| walker |  | 5534 | 85 | python decl doc at src/flask/views.py:16 |  |  | 0.529 |
| ns | 5665 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.521 |
| ns | 5726 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.525 |
| ns | 5925 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.519 |
| walker |  | 5985 | 451 | python class body at src/flask/views.py:16 |  |  | 0.519 |
| walker |  | 6036 | 51 | python method doc at src/flask/views.py:78 |  |  | 0.519 |
| walker |  | 6050 | 14 | python decl body at src/flask/json/__init__.py:77 body 105 |  |  | 0.519 |
| ns | 6069 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.512 |
| walker |  | 6113 | 63 | python method doc at src/flask/wrappers.py:246 |  |  | 0.512 |
| walker |  | 6125 | 12 | listing of 'tests/static' |  |  | 0.512 |
| walker |  | 6207 | 82 | python decl names surface in src/flask/testing.py |  |  | 0.517 |
| walker |  | 6207 | 0 | python decl at src/flask/testing.py:27 |  |  | 0.517 |
| walker |  | 6207 | 0 | python decl at src/flask/testing.py:100 |  |  | 0.517 |
| walker |  | 6207 | 0 | python decl at src/flask/testing.py:109 |  |  | 0.517 |
| walker |  | 6207 | 0 | python decl at src/flask/testing.py:265 |  |  | 0.517 |
| ns | 6212 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.510 |
| walker |  | 6218 | 11 | python class body at src/flask/testing.py:109 |  |  | 0.510 |
| ns | 6351 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.504 |
| walker |  | 6414 | 196 | python method sigs in src/flask/testing.py |  |  | 0.505 |
| walker |  | 6414 | 0 | python method at src/flask/testing.py:88 |  |  | 0.505 |
| walker |  | 6414 | 0 | python method at src/flask/testing.py:125 |  |  | 0.505 |
| walker |  | 6414 | 0 | python method at src/flask/testing.py:185 |  |  | 0.505 |
| walker |  | 6414 | 0 | python method at src/flask/testing.py:249 |  |  | 0.505 |
| walker |  | 6414 | 0 | python method at src/flask/testing.py:271 |  |  | 0.505 |
| walker |  | 6450 | 36 | python method at src/flask/testing.py:275 |  |  | 0.505 |
| walker |  | 6491 | 41 | python method at src/flask/testing.py:135 |  |  | 0.505 |
| ns | 6538 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.497 |
| walker |  | 6545 | 54 | python method at src/flask/testing.py:255 |  |  | 0.497 |
| walker |  | 6609 | 64 | python decl doc at src/flask/testing.py:265 |  |  | 0.497 |
| walker |  | 6672 | 63 | python method at src/flask/testing.py:204 |  |  | 0.497 |
| walker |  | 6705 | 33 | python method at src/flask/testing.py:193 |  |  | 0.497 |
| ns | 6757 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.487 |
| walker |  | 6764 | 59 | python method doc at src/flask/testing.py:88 |  |  | 0.487 |
| walker |  | 6864 | 100 | python method at src/flask/testing.py:49 |  |  | 0.487 |
| walker |  | 6919 | 55 | python imports in src/flask/globals.py |  |  | 0.487 |
| ns | 6942 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.479 |
| walker |  | 6989 | 70 | python decl names surface in src/flask/logging.py |  |  | 0.479 |
| walker |  | 6989 | 0 | python decl at src/flask/logging.py:31 |  |  | 0.479 |
| walker |  | 6989 | 0 | python decl at src/flask/logging.py:58 |  |  | 0.479 |
| walker |  | 7004 | 15 | python decl at src/flask/logging.py:15 |  |  | 0.479 |
| walker |  | 7053 | 49 | python decl doc at src/flask/logging.py:31 |  |  | 0.479 |
| walker |  | 7062 | 9 | python decl body at src/flask/logging.py:15 body 28 |  |  | 0.479 |
| ns | 7070 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.474 |
| walker |  | 7075 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.482 |
| walker |  | 7089 | 14 | python decl names surface in examples/javascript/js_example/__init__.py |  |  | 0.482 |
| walker |  | 7113 | 24 | python imports in examples/javascript/js_example/__init__.py |  |  | 0.482 |
| walker |  | 7127 | 14 | python decl names surface in tests/test_apps/blueprintapp/__init__.py |  |  | 0.482 |
| walker |  | 7277 | 150 | python decl names surface in src/flask/ctx.py |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:30 |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:154 |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:209 |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:235 |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:260 |  |  | 0.484 |
| walker |  | 7277 | 0 | python decl at src/flask/ctx.py:528 |  |  | 0.484 |
| ns | 7303 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.475 |
| walker |  | 7304 | 27 | python decl at src/flask/ctx.py:118 |  |  | 0.475 |
| walker |  | 7324 | 20 | python decl doc at src/flask/ctx.py:209 |  |  | 0.475 |
| walker |  | 7340 | 16 | python decl body at src/flask/ctx.py:235 body 257 |  |  | 0.475 |
| walker |  | 7363 | 23 | python decl body at src/flask/ctx.py:209 body 232 |  |  | 0.475 |
| ns | 7495 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.487 |
| ns | 7748 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.503 |
| walker |  | 7785 | 422 | python method sigs in src/flask/ctx.py |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:53 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:59 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:62 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:68 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:79 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:93 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:105 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:108 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:111 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:355 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:381 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:405 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:416 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:446 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:506 |  |  | 0.505 |
| walker |  | 7785 | 0 | python method at src/flask/ctx.py:518 |  |  | 0.505 |
| walker |  | 7793 | 8 | python method at src/flask/ctx.py:339 |  |  | 0.505 |
| walker |  | 7801 | 8 | python method at src/flask/ctx.py:350 |  |  | 0.505 |
| walker |  | 7809 | 8 | python method at src/flask/ctx.py:370 |  |  | 0.505 |
| walker |  | 7817 | 8 | python method at src/flask/ctx.py:395 |  |  | 0.505 |
| walker |  | 7833 | 16 | python method doc at src/flask/ctx.py:350 |  |  | 0.505 |
| walker |  | 7891 | 58 | python method at src/flask/ctx.py:300 |  |  | 0.505 |
| walker |  | 7949 | 58 | python method at src/flask/ctx.py:510 |  |  | 0.505 |
| walker |  | 8006 | 57 | python decl doc at src/flask/ctx.py:154 |  |  | 0.505 |
| walker |  | 8044 | 38 | python method doc at src/flask/ctx.py:405 |  |  | 0.505 |
| ns | 8047 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.494 |
| walker |  | 8064 | 20 | python method doc at src/flask/ctx.py:381 |  |  | 0.494 |
| walker |  | 8157 | 93 | python decl doc at src/flask/ctx.py:260 |  |  | 0.494 |
| ns | 8158 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.491 |
| walker |  | 8212 | 55 | python method doc at src/flask/ctx.py:370 |  |  | 0.491 |
| walker |  | 8275 | 63 | python method doc at src/flask/ctx.py:339 |  |  | 0.491 |
| walker |  | 8343 | 68 | python method doc at src/flask/ctx.py:395 |  |  | 0.491 |
| ns | 8373 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.484 |
| ns | 8515 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.493 |
| walker |  | 8601 | 258 | python decl names surface in src/flask/helpers.py |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:28 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:36 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:151 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:281 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:304 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:326 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:402 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:587 |  |  | 0.508 |
| walker |  | 8601 | 0 | python decl at src/flask/helpers.py:654 |  |  | 0.508 |
| walker |  | 8638 | 37 | python decl at src/flask/helpers.py:254 |  |  | 0.508 |
| walker |  | 8677 | 39 | python decl at src/flask/helpers.py:360 |  |  | 0.508 |
| walker |  | 8695 | 18 | python decl at src/flask/helpers.py:644 |  |  | 0.510 |
| walker |  | 8743 | 48 | python decl at src/flask/helpers.py:51 |  |  | 0.506 |
| walker |  | 8743 | 0 | python decl body at src/flask/helpers.py:51 body 54 |  |  | 0.506 |
| ns | 8743 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.506 |
| walker |  | 8792 | 49 | python decl at src/flask/helpers.py:543 |  |  | 0.506 |
| walker |  | 8812 | 20 | python decl doc at src/flask/helpers.py:543 |  |  | 0.506 |
| walker |  | 8867 | 55 | python decl at src/flask/helpers.py:63 |  |  | 0.506 |
| walker |  | 8926 | 59 | python decl at src/flask/helpers.py:57 |  |  | 0.508 |
| walker |  | 8926 | 0 | python decl body at src/flask/helpers.py:57 body 60 |  |  | 0.508 |
| ns | 8973 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.523 |
| walker |  | 8988 | 62 | python method sigs in src/flask/helpers.py |  |  | 0.523 |
| walker |  | 8988 | 0 | python method at src/flask/helpers.py:659 |  |  | 0.523 |
| walker |  | 8988 | 0 | python method at src/flask/helpers.py:662 |  |  | 0.523 |
| walker |  | 8988 | 0 | python method at src/flask/helpers.py:676 |  |  | 0.523 |
| walker |  | 9001 | 13 | python method doc at src/flask/helpers.py:676 |  |  | 0.523 |
| walker |  | 9051 | 50 | python decl doc at src/flask/helpers.py:28 |  |  | 0.523 |
| walker |  | 9141 | 90 | python decl at src/flask/helpers.py:200 |  |  | 0.523 |
| walker |  | 9159 | 18 | python decl doc at src/flask/helpers.py:200 |  |  | 0.523 |
| walker |  | 9218 | 59 | python method at src/flask/helpers.py:665 |  |  | 0.523 |
| ns | 9229 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.516 |
| walker |  | 9295 | 77 | python decl doc at src/flask/helpers.py:63 |  |  | 0.516 |
| walker |  | 9337 | 42 | python decl doc at src/flask/helpers.py:654 |  |  | 0.516 |
| ns | 9408 |  | 179 | Complete docs/ listing (35 entries) | 6.4 |  | 0.537 |
| walker |  | 9419 | 82 | python decl doc at src/flask/helpers.py:36 |  |  | 0.537 |
| walker |  | 9510 | 91 | python decl doc at src/flask/helpers.py:587 |  |  | 0.537 |
| ns | 9556 |  | 148 | Complete tests/ listing (27 entries) | 6.5 |  | 0.551 |
| walker |  | 9608 | 98 | python decl doc at src/flask/helpers.py:360 |  |  | 0.551 |
| ns | 9629 |  | 73 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.559 |
| walker |  | 9751 | 143 | python decl at src/flask/helpers.py:417 |  |  | 0.559 |
| walker |  | 9767 | 16 | python decl doc at src/flask/helpers.py:417 |  |  | 0.559 |
| walker |  | 9870 | 103 | python decl doc at src/flask/helpers.py:151 |  |  | 0.559 |
| ns | 9997 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.546 |

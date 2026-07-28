Score(3000)=0.686 I=0.893 C=0.526 ns_rows≤3K=18/57 (reached=9 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | listing of '.' |  |  | 0.000 |
| walker |  | 59 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 86 | 27 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| ns | 153 |  | 56 | Repository root listing (complete) | 1.2 |  | 0.484 |
| walker |  | 188 | 102 | README headline in README.md |  |  | 1.000 |
| walker |  | 218 | 30 | headings outline in README.md |  |  | 1.000 |
| walker |  | 228 | 10 | listing of '.devcontainer' |  |  | 1.000 |
| walker |  | 239 | 11 | listing of 'examples' |  |  | 1.000 |
| walker |  | 253 | 14 | listing of '.github' |  |  | 1.000 |
| walker |  | 275 | 22 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 277 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.592 |
| walker |  | 367 | 92 | listing of 'src/flask' |  |  | 0.836 |
| walker |  | 380 | 13 | python imports in src/flask/__init__.py |  |  | 0.836 |
| walker |  | 392 | 12 | python imports #1 in src/flask/__init__.py |  |  | 0.837 |
| ns | 401 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.763 |
| walker |  | 405 | 13 | python imports #2 in src/flask/__init__.py |  |  | 0.763 |
| walker |  | 417 | 12 | python imports #3 in src/flask/__init__.py |  |  | 0.764 |
| walker |  | 431 | 14 | python imports #7 in src/flask/__init__.py |  |  | 0.764 |
| walker |  | 444 | 13 | listing of 'src/flask/json' |  |  | 0.825 |
| walker |  | 460 | 16 | listing of 'src/flask/sansio' |  |  | 0.916 |
| walker |  | 526 | 66 | python imports #4 in src/flask/__init__.py |  |  | 0.922 |
| ns | 547 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.789 |
| walker |  | 576 | 50 | python imports #5 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 602 | 26 | python imports #10 in src/flask/__init__.py |  |  | 0.795 |
| walker |  | 666 | 64 | python imports in src/flask/json/__init__.py |  |  | 0.795 |
| ns | 713 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.810 |
| walker |  | 730 | 64 | python imports #9 in src/flask/__init__.py |  |  | 0.812 |
| ns | 871 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.756 |
| walker |  | 874 | 144 | python imports #6 in src/flask/__init__.py |  |  | 0.831 |
| walker |  | 886 | 12 | python imports in src/flask/__main__.py |  |  | 0.831 |
| walker |  | 1065 | 179 | listing of 'docs' |  |  | 0.835 |
| walker |  | 1091 | 26 | listing of 'docs/_static' |  |  | 0.835 |
| ns | 1127 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.774 |
| walker |  | 1154 | 63 | listing of 'docs/deploying' |  |  | 0.774 |
| walker |  | 1231 | 77 | listing of 'docs/tutorial' |  |  | 0.774 |
| walker |  | 1397 | 166 | python imports #8 in src/flask/__init__.py |  |  | 0.769 |
| ns | 1397 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.769 |
| ns | 1474 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.749 |
| walker |  | 1519 | 122 | README.md section #0 |  |  | 0.794 |
| ns | 1551 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.775 |
| walker |  | 1604 | 85 | [package] in pyproject.toml |  |  | 0.787 |
| walker |  | 1627 | 23 | tool.codespell config in pyproject.toml |  |  | 0.787 |
| ns | 1645 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.760 |
| walker |  | 1665 | 38 | package metadata in pyproject.toml |  |  | 0.771 |
| ns | 1751 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.746 |
| walker |  | 1788 | 123 | python decl names surface in src/flask/json/__init__.py |  |  | 0.746 |
| walker |  | 1788 | 0 | python decl at src/flask/json/__init__.py:13 |  |  | 0.746 |
| walker |  | 1788 | 0 | python decl at src/flask/json/__init__.py:47 |  |  | 0.746 |
| walker |  | 1788 | 0 | python decl at src/flask/json/__init__.py:77 |  |  | 0.746 |
| walker |  | 1788 | 0 | python decl at src/flask/json/__init__.py:108 |  |  | 0.746 |
| walker |  | 1788 | 0 | python decl at src/flask/json/__init__.py:138 |  |  | 0.746 |
| walker |  | 1799 | 11 | python decl doc at src/flask/json/__init__.py:13 |  |  | 0.746 |
| walker |  | 1810 | 11 | python decl doc at src/flask/json/__init__.py:77 |  |  | 0.746 |
| walker |  | 1825 | 15 | python decl doc at src/flask/json/__init__.py:108 |  |  | 0.746 |
| walker |  | 1841 | 16 | python decl doc at src/flask/json/__init__.py:47 |  |  | 0.746 |
| ns | 1902 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.713 |
| walker |  | 1986 | 145 | [dependencies] in pyproject.toml |  |  | 0.777 |
| walker |  | 2011 | 25 | python decl body at src/flask/json/__init__.py:138 body 170 |  |  | 0.777 |
| ns | 2118 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.739 |
| walker |  | 2155 | 144 | listing of 'docs/patterns' |  |  | 0.739 |
| walker |  | 2179 | 24 | listing of 'examples/celery' |  |  | 0.739 |
| walker |  | 2183 | 4 | listing of 'examples/celery/src' |  |  | 0.739 |
| walker |  | 2200 | 17 | listing of 'examples/celery/src/task_app' |  |  | 0.739 |
| ns | 2300 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.711 |
| walker |  | 2348 | 148 | listing of 'tests' |  |  | 0.714 |
| walker |  | 2366 | 18 | listing of 'tests/type_check' |  |  | 0.714 |
| walker |  | 2443 | 77 | README headline in src/flask/sansio/README.md |  |  | 0.735 |
| walker |  | 2471 | 28 | listing of 'examples/javascript' |  |  | 0.735 |
| walker |  | 2484 | 13 | listing of 'examples/javascript/js_example' |  |  | 0.735 |
| walker |  | 2512 | 28 | listing of 'examples/tutorial' |  |  | 0.735 |
| ns | 2516 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.706 |
| walker |  | 2646 | 134 | manifest config in pyproject.toml |  |  | 0.706 |
| ns | 2661 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.685 |
| walker |  | 2720 | 74 | python decl doc at src/flask/json/__init__.py:138 |  |  | 0.685 |
| walker |  | 2749 | 29 | listing of 'examples/tutorial/tests' |  |  | 0.685 |
| walker |  | 2781 | 32 | listing of 'tests/test_apps' |  |  | 0.685 |
| walker |  | 2790 | 9 | listing of 'tests/test_apps/blueprintapp' |  |  | 0.685 |
| walker |  | 2799 | 9 | listing of 'tests/test_apps/subdomaintestmodule' |  |  | 0.685 |
| walker |  | 2812 | 13 | listing of 'tests/test_apps/blueprintapp/apps' |  |  | 0.685 |
| walker |  | 2821 | 9 | listing of 'tests/test_apps/blueprintapp/apps/frontend' |  |  | 0.685 |
| walker |  | 2834 | 13 | listing of 'tests/test_apps/blueprintapp/apps/admin' |  |  | 0.685 |
| walker |  | 2863 | 29 | listing of 'examples/tutorial/flaskr' |  |  | 0.686 |
| walker |  | 2896 | 33 | listing of 'tests/test_apps/cliapp' |  |  | 0.686 |
| walker |  | 2906 | 10 | listing of 'tests/test_apps/cliapp/inner1' |  |  | 0.686 |
| walker |  | 2915 | 9 | listing of 'tests/test_apps/cliapp/inner1/inner2' |  |  | 0.686 |
| walker |  | 2990 | 75 | README.md section #3 |  |  | 0.686 |
| ns | 3091 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.635 |
| walker |  | 3183 | 193 | tool.mypy+pyright config in pyproject.toml |  |  | 0.636 |
| ns | 3252 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.618 |
| walker |  | 3379 | 196 | tool.ruff config in pyproject.toml |  |  | 0.618 |
| walker |  | 3469 | 90 | README.md section #2 |  |  | 0.618 |
| ns | 3594 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.592 |
| walker |  | 3604 | 135 | README.md section #1 |  |  | 0.646 |
| ns | 3762 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.631 |
| walker |  | 3928 | 324 | tool.flit+uv+pytest+coverage config in pyproject.toml |  |  | 0.632 |
| ns | 3947 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.618 |
| ns | 3999 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.611 |
| ns | 4094 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.617 |
| walker |  | 4255 | 327 | dev/build/target dependencies in pyproject.toml |  |  | 0.617 |
| walker |  | 4266 | 11 | python imports in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.617 |
| walker |  | 4277 | 11 | python imports in tests/test_apps/subdomaintestmodule/__init__.py |  |  | 0.617 |
| ns | 4303 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.598 |
| walker |  | 4305 | 28 | python imports in src/flask/signals.py |  |  | 0.599 |
| walker |  | 4313 | 8 | python decl names surface in tests/test_apps/cliapp/inner1/__init__.py |  |  | 0.599 |
| walker |  | 4337 | 24 | python decl names surface in src/flask/wrappers.py |  |  | 0.599 |
| walker |  | 4337 | 0 | python decl at src/flask/wrappers.py:18 |  |  | 0.599 |
| walker |  | 4337 | 0 | python decl at src/flask/wrappers.py:222 |  |  | 0.599 |
| ns | 4515 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.582 |
| walker |  | 4571 | 234 | python method sigs in src/flask/wrappers.py |  |  | 0.582 |
| walker |  | 4571 | 0 | python method at src/flask/wrappers.py:197 |  |  | 0.582 |
| walker |  | 4571 | 0 | python method at src/flask/wrappers.py:212 |  |  | 0.582 |
| walker |  | 4579 | 8 | python method at src/flask/wrappers.py:59 |  |  | 0.582 |
| walker |  | 4587 | 8 | python method at src/flask/wrappers.py:92 |  |  | 0.582 |
| walker |  | 4595 | 8 | python method at src/flask/wrappers.py:146 |  |  | 0.582 |
| walker |  | 4603 | 8 | python method at src/flask/wrappers.py:161 |  |  | 0.582 |
| walker |  | 4611 | 8 | python method at src/flask/wrappers.py:180 |  |  | 0.582 |
| walker |  | 4619 | 8 | python method at src/flask/wrappers.py:246 |  |  | 0.582 |
| walker |  | 4630 | 11 | python method at src/flask/wrappers.py:88 |  |  | 0.582 |
| walker |  | 4641 | 11 | python method at src/flask/wrappers.py:142 |  |  | 0.582 |
| walker |  | 4653 | 12 | python method at src/flask/wrappers.py:115 |  |  | 0.582 |
| walker |  | 4668 | 15 | python method at src/flask/wrappers.py:119 |  |  | 0.582 |
| ns | 4670 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.600 |
| walker |  | 4717 | 49 | python class body at src/flask/wrappers.py:222 |  |  | 0.600 |
| ns | 4854 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.585 |
| ns | 4955 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.580 |
| ns | 5103 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.573 |
| walker |  | 5121 | 404 | python class body at src/flask/wrappers.py:18 |  |  | 0.574 |
| walker |  | 5181 | 60 | python decl body at src/flask/json/__init__.py:47 body 70 |  |  | 0.574 |
| walker |  | 5184 | 3 | listing of 'examples/tutorial/flaskr/static' |  |  | 0.574 |
| walker |  | 5187 | 3 | listing of 'tests/test_apps/subdomaintestmodule/static' |  |  | 0.574 |
| walker |  | 5226 | 39 | python imports in src/flask/typing.py |  |  | 0.574 |
| walker |  | 5241 | 15 | python decl names surface in src/flask/blueprints.py |  |  | 0.574 |
| walker |  | 5241 | 0 | python decl at src/flask/blueprints.py:18 |  |  | 0.574 |
| ns | 5247 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.566 |
| walker |  | 5307 | 66 | python method sigs in src/flask/blueprints.py |  |  | 0.566 |
| walker |  | 5307 | 0 | python method at src/flask/blueprints.py:55 |  |  | 0.566 |
| walker |  | 5307 | 0 | python method at src/flask/blueprints.py:82 |  |  | 0.566 |
| walker |  | 5349 | 42 | python method at src/flask/blueprints.py:104 |  |  | 0.566 |
| walker |  | 5370 | 21 | python imports in tests/test_apps/blueprintapp/apps/admin/__init__.py |  |  | 0.566 |
| walker |  | 5391 | 21 | python imports in tests/test_apps/blueprintapp/apps/frontend/__init__.py |  |  | 0.566 |
| walker |  | 5452 | 61 | python decl names surface in src/flask/views.py |  |  | 0.567 |
| walker |  | 5452 | 0 | python decl at src/flask/views.py:16 |  |  | 0.567 |
| walker |  | 5452 | 0 | python decl at src/flask/views.py:138 |  |  | 0.567 |
| walker |  | 5484 | 32 | python decl at src/flask/views.py:11 |  |  | 0.567 |
| ns | 5493 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.553 |
| walker |  | 5534 | 50 | python decl doc at src/flask/views.py:138 |  |  | 0.553 |
| walker |  | 5611 | 77 | python method sigs in src/flask/views.py |  |  | 0.553 |
| walker |  | 5611 | 0 | python method at src/flask/views.py:78 |  |  | 0.553 |
| walker |  | 5611 | 0 | python method at src/flask/views.py:165 |  |  | 0.553 |
| walker |  | 5611 | 0 | python method at src/flask/views.py:182 |  |  | 0.553 |
| walker |  | 5623 | 12 | python method body at src/flask/views.py:78 body 83 |  |  | 0.553 |
| walker |  | 5665 | 42 | python method at src/flask/views.py:85 |  |  | 0.544 |
| ns | 5665 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.544 |
| ns | 5726 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.548 |
| walker |  | 5750 | 85 | python decl doc at src/flask/views.py:16 |  |  | 0.548 |
| ns | 5925 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.541 |
| ns | 6069 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.534 |
| walker |  | 6201 | 451 | python class body at src/flask/views.py:16 |  |  | 0.535 |
| ns | 6212 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.528 |
| walker |  | 6252 | 51 | python method doc at src/flask/views.py:78 |  |  | 0.528 |
| walker |  | 6301 | 49 | declaration surface of docs/reqcontext.rst |  |  | 0.528 |
| walker |  | 6313 | 12 | listing of 'tests/static' |  |  | 0.528 |
| walker |  | 6336 | 23 | python imports in examples/tutorial/flaskr/__init__.py |  |  | 0.528 |
| walker |  | 6347 | 11 | python decl names surface in examples/tutorial/flaskr/__init__.py |  |  | 0.528 |
| walker |  | 6347 | 0 | python decl at examples/tutorial/flaskr/__init__.py:6 |  |  | 0.528 |
| ns | 6351 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.521 |
| walker |  | 6429 | 82 | python decl names surface in src/flask/testing.py |  |  | 0.522 |
| walker |  | 6429 | 0 | python decl at src/flask/testing.py:27 |  |  | 0.522 |
| walker |  | 6429 | 0 | python decl at src/flask/testing.py:100 |  |  | 0.522 |
| walker |  | 6429 | 0 | python decl at src/flask/testing.py:109 |  |  | 0.522 |
| walker |  | 6429 | 0 | python decl at src/flask/testing.py:265 |  |  | 0.522 |
| walker |  | 6440 | 11 | python class body at src/flask/testing.py:109 |  |  | 0.522 |
| ns | 6538 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.514 |
| walker |  | 6636 | 196 | python method sigs in src/flask/testing.py |  |  | 0.514 |
| walker |  | 6636 | 0 | python method at src/flask/testing.py:88 |  |  | 0.514 |
| walker |  | 6636 | 0 | python method at src/flask/testing.py:125 |  |  | 0.514 |
| walker |  | 6636 | 0 | python method at src/flask/testing.py:185 |  |  | 0.514 |
| walker |  | 6636 | 0 | python method at src/flask/testing.py:249 |  |  | 0.514 |
| walker |  | 6636 | 0 | python method at src/flask/testing.py:271 |  |  | 0.514 |
| walker |  | 6672 | 36 | python method at src/flask/testing.py:275 |  |  | 0.514 |
| walker |  | 6713 | 41 | python method at src/flask/testing.py:135 |  |  | 0.514 |
| ns | 6757 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.505 |
| walker |  | 6767 | 54 | python method at src/flask/testing.py:255 |  |  | 0.505 |
| walker |  | 6831 | 64 | python decl doc at src/flask/testing.py:265 |  |  | 0.505 |
| walker |  | 6894 | 63 | python method at src/flask/testing.py:204 |  |  | 0.505 |
| walker |  | 6927 | 33 | python method at src/flask/testing.py:193 |  |  | 0.505 |
| ns | 6942 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.496 |
| walker |  | 6986 | 59 | python method doc at src/flask/testing.py:88 |  |  | 0.496 |
| ns | 7070 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.491 |
| walker |  | 7244 | 258 | python decl names surface in src/flask/helpers.py |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:28 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:36 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:151 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:281 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:304 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:326 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:402 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:587 |  |  | 0.508 |
| walker |  | 7244 | 0 | python decl at src/flask/helpers.py:654 |  |  | 0.508 |
| walker |  | 7281 | 37 | python decl at src/flask/helpers.py:254 |  |  | 0.508 |
| ns | 7303 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.498 |
| walker |  | 7320 | 39 | python decl at src/flask/helpers.py:360 |  |  | 0.498 |
| walker |  | 7338 | 18 | python decl at src/flask/helpers.py:644 |  |  | 0.501 |
| walker |  | 7386 | 48 | python decl at src/flask/helpers.py:51 |  |  | 0.504 |
| walker |  | 7386 | 0 | python decl body at src/flask/helpers.py:51 body 54 |  |  | 0.504 |
| walker |  | 7435 | 49 | python decl at src/flask/helpers.py:543 |  |  | 0.504 |
| walker |  | 7455 | 20 | python decl doc at src/flask/helpers.py:543 |  |  | 0.504 |
| ns | 7495 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.515 |
| walker |  | 7510 | 55 | python decl at src/flask/helpers.py:63 |  |  | 0.515 |
| walker |  | 7569 | 59 | python decl at src/flask/helpers.py:57 |  |  | 0.518 |
| walker |  | 7569 | 0 | python decl body at src/flask/helpers.py:57 body 60 |  |  | 0.518 |
| walker |  | 7631 | 62 | python method sigs in src/flask/helpers.py |  |  | 0.518 |
| walker |  | 7631 | 0 | python method at src/flask/helpers.py:659 |  |  | 0.518 |
| walker |  | 7631 | 0 | python method at src/flask/helpers.py:662 |  |  | 0.518 |
| walker |  | 7631 | 0 | python method at src/flask/helpers.py:676 |  |  | 0.518 |
| walker |  | 7644 | 13 | python method doc at src/flask/helpers.py:676 |  |  | 0.518 |
| walker |  | 7694 | 50 | python decl doc at src/flask/helpers.py:28 |  |  | 0.518 |
| ns | 7748 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.532 |
| walker |  | 7784 | 90 | python decl at src/flask/helpers.py:200 |  |  | 0.532 |
| walker |  | 7802 | 18 | python decl doc at src/flask/helpers.py:200 |  |  | 0.532 |
| walker |  | 7861 | 59 | python method at src/flask/helpers.py:665 |  |  | 0.532 |
| walker |  | 7938 | 77 | python decl doc at src/flask/helpers.py:63 |  |  | 0.532 |
| walker |  | 7980 | 42 | python decl doc at src/flask/helpers.py:654 |  |  | 0.532 |
| ns | 8047 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.521 |
| walker |  | 8062 | 82 | python decl doc at src/flask/helpers.py:36 |  |  | 0.521 |
| walker |  | 8153 | 91 | python decl doc at src/flask/helpers.py:587 |  |  | 0.521 |
| ns | 8158 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.517 |
| walker |  | 8251 | 98 | python decl doc at src/flask/helpers.py:360 |  |  | 0.517 |
| ns | 8373 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.510 |
| walker |  | 8394 | 143 | python decl at src/flask/helpers.py:417 |  |  | 0.510 |
| walker |  | 8410 | 16 | python decl doc at src/flask/helpers.py:417 |  |  | 0.510 |
| walker |  | 8513 | 103 | python decl doc at src/flask/helpers.py:151 |  |  | 0.510 |
| ns | 8515 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.519 |
| walker |  | 8663 | 150 | python decl names surface in src/flask/ctx.py |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:30 |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:154 |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:209 |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:235 |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:260 |  |  | 0.520 |
| walker |  | 8663 | 0 | python decl at src/flask/ctx.py:528 |  |  | 0.520 |
| walker |  | 8690 | 27 | python decl at src/flask/ctx.py:118 |  |  | 0.520 |
| walker |  | 8710 | 20 | python decl doc at src/flask/ctx.py:209 |  |  | 0.520 |
| walker |  | 8726 | 16 | python decl body at src/flask/ctx.py:235 body 257 |  |  | 0.520 |
| ns | 8743 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.513 |
| walker |  | 8749 | 23 | python decl body at src/flask/ctx.py:209 body 232 |  |  | 0.513 |
| ns | 8973 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.507 |
| walker |  | 9171 | 422 | python method sigs in src/flask/ctx.py |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:53 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:59 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:62 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:68 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:79 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:93 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:105 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:108 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:111 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:355 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:381 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:405 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:416 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:446 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:506 |  |  | 0.528 |
| walker |  | 9171 | 0 | python method at src/flask/ctx.py:518 |  |  | 0.528 |
| walker |  | 9179 | 8 | python method at src/flask/ctx.py:339 |  |  | 0.528 |
| walker |  | 9187 | 8 | python method at src/flask/ctx.py:350 |  |  | 0.528 |
| walker |  | 9195 | 8 | python method at src/flask/ctx.py:370 |  |  | 0.528 |
| walker |  | 9203 | 8 | python method at src/flask/ctx.py:395 |  |  | 0.528 |
| walker |  | 9219 | 16 | python method doc at src/flask/ctx.py:350 |  |  | 0.528 |
| ns | 9229 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.522 |
| walker |  | 9277 | 58 | python method at src/flask/ctx.py:300 |  |  | 0.522 |
| walker |  | 9335 | 58 | python method at src/flask/ctx.py:510 |  |  | 0.522 |
| walker |  | 9392 | 57 | python decl doc at src/flask/ctx.py:154 |  |  | 0.522 |
| ns | 9408 |  | 179 | Complete docs/ listing (35 entries) | 6.4 |  | 0.542 |
| walker |  | 9430 | 38 | python method doc at src/flask/ctx.py:405 |  |  | 0.542 |
| walker |  | 9450 | 20 | python method doc at src/flask/ctx.py:381 |  |  | 0.542 |
| walker |  | 9543 | 93 | python decl doc at src/flask/ctx.py:260 |  |  | 0.542 |
| ns | 9556 |  | 148 | Complete tests/ listing (27 entries) | 6.5 |  | 0.556 |
| walker |  | 9598 | 55 | python method doc at src/flask/ctx.py:370 |  |  | 0.556 |
| walker |  | 9612 | 14 | python decl body at src/flask/json/__init__.py:77 body 105 |  |  | 0.556 |
| ns | 9629 |  | 73 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.564 |
| walker |  | 9682 | 70 | python decl names surface in src/flask/logging.py |  |  | 0.564 |
| walker |  | 9682 | 0 | python decl at src/flask/logging.py:31 |  |  | 0.564 |
| walker |  | 9682 | 0 | python decl at src/flask/logging.py:58 |  |  | 0.564 |
| walker |  | 9697 | 15 | python decl at src/flask/logging.py:15 |  |  | 0.564 |
| walker |  | 9746 | 49 | python decl doc at src/flask/logging.py:31 |  |  | 0.564 |
| walker |  | 9755 | 9 | python decl body at src/flask/logging.py:15 body 28 |  |  | 0.564 |
| walker |  | 9939 | 184 | python decl names surface in src/flask/signals.py |  |  | 0.577 |
| walker |  | 9952 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.582 |
| ns | 9997 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.569 |

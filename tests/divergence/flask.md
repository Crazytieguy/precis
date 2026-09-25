Score(3000)=0.674 I=0.886 C=0.513 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.773/0.664/0.778/0.674/0.552/0.509/0.534

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 53 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 62 | 9 | Fs::DirListing { dir: examples } |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| ns | 147 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.484 |
| walker |  | 149 | 87 | Toml::Identity { file: pyproject.toml } |  |  | 0.488 |
| walker |  | 160 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.489 |
| walker |  | 185 | 25 | Toml::Operational { file: pyproject.toml } |  |  | 0.489 |
| walker |  | 198 | 13 | Fs::DirListing { dir: .github } |  |  | 0.490 |
| walker |  | 221 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.492 |
| ns | 271 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.291 |
| walker |  | 323 | 102 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.594 |
| walker |  | 353 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.594 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.542 |
| walker |  | 446 | 93 | Fs::DirListing { dir: src/flask } |  |  | 0.765 |
| walker |  | 460 | 14 | Fs::DirListing { dir: src/flask/json } |  |  | 0.826 |
| walker |  | 477 | 17 | Fs::DirListing { dir: src/flask/sansio } |  |  | 0.916 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.784 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.716 |
| walker |  | 717 | 240 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.814 |
| walker |  | 732 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.773 |
| walker |  | 881 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| walker |  | 906 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.773 |
| walker |  | 930 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| walker |  | 1106 | 176 | Fs::DirListing { dir: docs } |  |  | 0.776 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.709 |
| walker |  | 1133 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.709 |
| walker |  | 1197 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.709 |
| walker |  | 1235 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.711 |
| walker |  | 1313 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.711 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.664 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.647 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.633 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.611 |
| walker |  | 1653 | 340 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.730 |
| walker |  | 1696 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.730 |
| walker |  | 1739 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.730 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.706 |
| walker |  | 1884 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.774 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.740 |
| walker |  | 1940 | 56 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.740 |
| walker |  | 2062 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.778 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.740 |
| walker |  | 2207 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.740 |
| walker |  | 2263 | 56 | Code::CodeKey { rung: Names, file: src/flask/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.740 |
| walker |  | 2287 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.740 |
| walker |  | 2291 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.740 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.716 |
| walker |  | 2308 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.716 |
| walker |  | 2369 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| walker |  | 2401 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.720 |
| walker |  | 2449 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.720 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.691 |
| walker |  | 2594 | 145 | Fs::DirListing { dir: tests } |  |  | 0.694 |
| walker |  | 2613 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.694 |
| walker |  | 2640 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.694 |
| walker |  | 2653 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.694 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.673 |
| walker |  | 2680 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.673 |
| walker |  | 2740 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.673 |
| walker |  | 2806 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.673 |
| walker |  | 2835 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.673 |
| walker |  | 2844 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.673 |
| walker |  | 2853 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.673 |
| walker |  | 2867 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.673 |
| walker |  | 2876 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.673 |
| walker |  | 2888 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.673 |
| walker |  | 2931 | 43 | Code::CodeKey { rung: Names, file: src/flask/json/provider.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 2961 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.674 |
| walker |  | 3034 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.674 |
| walker |  | 3040 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.674 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.625 |
| walker |  | 3117 | 77 | Code::CodeKey { rung: Names, file: src/flask/logging.py, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 3125 | 8 | Code::CodeKey { rung: Decl, file: src/flask/logging.py, decl: 1, sub: 0, line: 15 } |  |  | 0.625 |
| walker |  | 3207 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 3245 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.626 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.608 |
| walker |  | 3287 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.608 |
| walker |  | 3323 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.608 |
| walker |  | 3351 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.609 |
| walker |  | 3393 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.609 |
| walker |  | 3470 | 77 | Markdown::ReadmeHeadline { file: src/flask/sansio/README.md } |  |  | 0.626 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.601 |
| walker |  | 3719 | 249 | Code::CodeKey { rung: Names, file: src/flask/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 3745 | 26 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 11, sub: 0, line: 293 } |  |  | 0.603 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.589 |
| walker |  | 3781 | 36 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 8, sub: 0, line: 241 } |  |  | 0.589 |
| walker |  | 3829 | 48 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 7, sub: 0, line: 235 } |  |  | 0.589 |
| walker |  | 3876 | 47 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 6, sub: 0, line: 229 } |  |  | 0.589 |
| walker |  | 3937 | 61 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 10, sub: 0, line: 283 } |  |  | 0.589 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.575 |
| walker |  | 3972 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.575 |
| walker |  | 3982 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.575 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.569 |
| walker |  | 3994 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.569 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.561 |
| walker |  | 4100 | 106 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 4120 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.569 |
| walker |  | 4160 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.569 |
| walker |  | 4167 | 7 | Code::CodeKey { rung: Body, file: src/flask/debughelpers.py, decl: 4, sub: 0, line: 46 } |  |  | 0.569 |
| walker |  | 4261 | 94 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.569 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.552 |
| walker |  | 4383 | 122 | Code::CodeKey { rung: Names, file: src/flask/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4451 | 68 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 6, sub: 0, line: 83 } |  |  | 0.570 |
| walker |  | 4465 | 14 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.570 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.553 |
| walker |  | 4590 | 125 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 4, sub: 0, line: 57 } |  |  | 0.553 |
| walker |  | 4625 | 35 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 5, sub: 0, line: 73 } |  |  | 0.553 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.539 |
| walker |  | 4755 | 130 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.543 |
| walker |  | 4763 | 8 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 5, sub: 0, line: 32 } |  |  | 0.543 |
| walker |  | 4773 | 10 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 4, sub: 0, line: 29 } |  |  | 0.543 |
| walker |  | 4785 | 12 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.543 |
| walker |  | 4822 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 3, sub: 0, line: 23 } |  |  | 0.543 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.529 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.525 |
| walker |  | 4954 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.525 |
| walker |  | 4995 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.525 |
| walker |  | 5049 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.525 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.519 |
| walker |  | 5112 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.519 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.512 |
| walker |  | 5245 | 133 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 8, sub: 0, line: 50 } |  |  | 0.536 |
| walker |  | 5276 | 31 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 12, sub: 0, line: 187 } |  |  | 0.536 |
| walker |  | 5312 | 36 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.536 |
| walker |  | 5349 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 16, sub: 0, line: 323 } |  |  | 0.536 |
| walker |  | 5389 | 40 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 11, sub: 0, line: 126 } |  |  | 0.536 |
| walker |  | 5438 | 49 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 9, sub: 0, line: 94 } |  |  | 0.536 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.524 |
| walker |  | 5513 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.524 |
| walker |  | 5588 | 75 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 14, sub: 0, line: 256 } |  |  | 0.524 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.515 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.520 |
| walker |  | 5738 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 5765 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.525 |
| walker |  | 5781 | 16 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 16, sub: 0, line: 235 } |  |  | 0.525 |
| walker |  | 5804 | 23 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 15, sub: 0, line: 209 } |  |  | 0.525 |
| walker |  | 5894 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.525 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.519 |
| walker |  | 5971 | 77 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 12, sub: 0, line: 305 } |  |  | 0.519 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.512 |
| walker |  | 6126 | 155 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 6152 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.516 |
| walker |  | 6190 | 38 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.516 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.509 |
| walker |  | 6230 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.509 |
| walker |  | 6335 | 105 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.509 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.503 |
| walker |  | 6375 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.503 |
| walker |  | 6407 | 32 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.503 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.495 |
| walker |  | 6698 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 6705 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.498 |
| walker |  | 6742 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.498 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.489 |
| walker |  | 6779 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.489 |
| walker |  | 6818 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.489 |
| walker |  | 6866 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.489 |
| walker |  | 6915 | 49 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.489 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.504 |
| walker |  | 6970 | 55 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.504 |
| walker |  | 7060 | 90 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.504 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.499 |
| walker |  | 7203 | 143 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.499 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.490 |
| walker |  | 7389 | 186 | Code::CodeKey { rung: Names, file: src/flask/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.501 |
| walker |  | 7576 | 187 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 7596 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 3, sub: 0, line: 41 } |  |  | 0.502 |
| walker |  | 7619 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 4, sub: 0, line: 44 } |  |  | 0.502 |
| walker |  | 7642 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 5, sub: 0, line: 47 } |  |  | 0.502 |
| walker |  | 7665 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 7, sub: 0, line: 57 } |  |  | 0.502 |
| walker |  | 7688 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 60 } |  |  | 0.502 |
| walker |  | 7738 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 6, sub: 0, line: 51 } |  |  | 0.502 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.495 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.486 |
| walker |  | 8061 | 323 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 8096 | 35 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 19, sub: 0, line: 84 } |  |  | 0.486 |
| walker |  | 8133 | 37 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.486 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.482 |
| walker |  | 8172 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.482 |
| walker |  | 8211 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 7, sub: 0, line: 50 } |  |  | 0.482 |
| walker |  | 8252 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 9, sub: 0, line: 55 } |  |  | 0.482 |
| walker |  | 8293 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.482 |
| walker |  | 8334 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 18, sub: 0, line: 79 } |  |  | 0.482 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.476 |
| walker |  | 8397 | 63 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.476 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.485 |
| walker |  | 8516 | 119 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.485 |
| walker |  | 8651 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.515 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.526 |
| walker |  | 8751 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.526 |
| walker |  | 8871 | 120 | Code::CodeKey { rung: Names, file: src/flask/json/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 8934 | 63 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 11, sub: 0, line: 119 } |  |  | 0.541 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.534 |
| walker |  | 8997 | 63 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 18, sub: 0, line: 147 } |  |  | 0.534 |
| walker |  | 9081 | 84 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 14, sub: 0, line: 133 } |  |  | 0.534 |
| walker |  | 9088 | 7 | Code::CodeKey { rung: Body, file: src/flask/json/tag.py, decl: 17, sub: 0, line: 143 } |  |  | 0.534 |
| walker |  | 9172 | 84 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 21, sub: 0, line: 159 } |  |  | 0.534 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.527 |
| walker |  | 9256 | 84 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 29, sub: 0, line: 191 } |  |  | 0.527 |
| walker |  | 9340 | 84 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 33, sub: 0, line: 205 } |  |  | 0.527 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.548 |
| walker |  | 9426 | 86 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 7, sub: 0, line: 93 } |  |  | 0.548 |
| walker |  | 9512 | 86 | Code::CodeKey { rung: Decl, file: src/flask/json/tag.py, decl: 25, sub: 0, line: 173 } |  |  | 0.548 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.562 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.570 |
| walker |  | 9719 | 207 | Code::CodeKey { rung: Names, file: src/flask/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 9742 | 23 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 1, sub: 0, line: 64 } |  |  | 0.571 |
| walker |  | 9804 | 62 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.571 |
| walker |  | 9837 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.571 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.557 |

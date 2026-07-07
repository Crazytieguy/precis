Score(3000)=0.510 I=0.801 C=0.324 ns_rows≤3K=33/52 (reached=12 partial=1 missing=20)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | listing of '.' |  |  | 1.000 |
| ns | 56 |  | 56 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 81 | 25 | listing of 'py3xui' |  |  | 1.000 |
| ns | 81 |  | 25 | py3xui/ package listing | 1.2 |  | 1.000 |
| ns | 121 |  | 40 | py3xui/api/ listing | 1.3 |  | 0.823 |
| ns | 167 |  | 46 | py3xui/async_api/ listing | 1.4 |  | 0.711 |
| walker |  | 175 | 94 | python imports in py3xui/__init__.py |  |  | 0.718 |
| ns | 181 |  | 14 | py3xui/client/ listing | 1.5 |  | 0.685 |
| walker |  | 189 | 14 | listing of 'py3xui/client' |  |  | 0.737 |
| walker |  | 203 | 14 | listing of 'py3xui/server' |  |  | 0.743 |
| ns | 213 |  | 32 | py3xui/inbound/ listing | 1.6 |  | 0.675 |
| walker |  | 217 | 14 | listing of 'py3xui/utils' |  |  | 0.679 |
| ns | 227 |  | 14 | py3xui/server/ listing | 1.7 |  | 0.693 |
| ns | 241 |  | 14 | py3xui/utils/ listing | 1.8 |  | 0.704 |
| walker |  | 245 | 28 | python imports in py3xui/client/__init__.py |  |  | 0.704 |
| walker |  | 262 | 17 | python decl names surface in py3xui/utils/__init__.py |  |  | 0.704 |
| ns | 266 |  | 25 | tests/ listing | 1.9 |  | 0.667 |
| walker |  | 293 | 31 | python imports in py3xui/utils/__init__.py |  |  | 0.669 |
| ns | 300 |  | 34 | tests/responses/ listing | 1.10 |  | 0.637 |
| walker |  | 325 | 32 | python imports in py3xui/server/__init__.py |  |  | 0.637 |
| ns | 327 |  | 27 | dev/ listing | 1.11 |  | 0.609 |
| ns | 345 |  | 18 | .github/ listing | 1.12 |  | 0.588 |
| walker |  | 357 | 32 | listing of 'py3xui/inbound' |  |  | 0.664 |
| ns | 368 |  | 23 | .github/workflows/ listing | 1.13 |  | 0.638 |
| ns | 382 |  | 14 | .github/ISSUE_TEMPLATE/ listing | 1.14 |  | 0.629 |
| ns | 394 |  | 12 | .vscode/ listing | 1.15 |  | 0.615 |
| walker |  | 397 | 40 | listing of 'py3xui/api' |  |  | 0.701 |
| walker |  | 443 | 46 | listing of 'py3xui/async_api' |  |  | 0.779 |
| walker |  | 454 | 11 | python decl names surface in py3xui/api/api.py |  |  | 0.779 |
| walker |  | 454 | 0 | python decl at py3xui/api/api.py:13 |  |  | 0.779 |
| walker |  | 466 | 12 | python decl names surface in py3xui/async_api/async_api.py |  |  | 0.779 |
| walker |  | 466 | 0 | python decl at py3xui/async_api/async_api.py:18 |  |  | 0.779 |
| ns | 488 |  | 94 | py3xui/__init__.py -- public export surface | 2.1 |  | 0.788 |
| walker |  | 546 | 80 | python imports in py3xui/api/__init__.py |  |  | 0.788 |
| ns | 603 |  | 115 | README Examples index (per-package doc pointers) | 2.2 |  | 0.753 |
| walker |  | 628 | 82 | python imports in py3xui/inbound/__init__.py |  |  | 0.753 |
| walker |  | 655 | 27 | listing of 'dev' |  |  | 0.790 |
| ns | 668 |  | 65 | Api: method/property locations | 2.3 |  | 0.762 |
| walker |  | 746 | 91 | python imports in py3xui/async_api/__init__.py |  |  | 0.732 |
| ns | 746 |  | 78 | InboundApi: method locations | 2.4 |  | 0.732 |
| ns | 783 |  | 37 | ServerApi: method locations | 2.5 |  | 0.721 |
| walker |  | 807 | 61 | [dependencies] in pyproject.toml |  |  | 0.721 |
| walker |  | 871 | 64 | README headline in py3xui/inbound/README.md |  |  | 0.721 |
| ns | 896 |  | 113 | ClientApi: method locations | 2.6 |  | 0.685 |
| walker |  | 937 | 66 | README headline in py3xui/api/README.md |  |  | 0.685 |
| walker |  | 1004 | 67 | README headline in py3xui/utils/README.md |  |  | 0.685 |
| ns | 1007 |  | 111 | BaseApi: ApiFields constants | 2.7 |  | 0.654 |
| walker |  | 1027 | 23 | python decl names surface in py3xui/client/client.py |  |  | 0.654 |
| walker |  | 1027 | 0 | python decl at py3xui/client/client.py:7 |  |  | 0.654 |
| walker |  | 1027 | 0 | python decl at py3xui/client/client.py:36 |  |  | 0.654 |
| ns | 1060 |  | 53 | utils/__init__.py: COOKIE_NAMES | 2.8 |  | 0.657 |
| walker |  | 1099 | 72 | README headline in py3xui/server/README.md |  |  | 0.657 |
| walker |  | 1172 | 73 | README headline in py3xui/client/README.md |  |  | 0.657 |
| ns | 1174 |  | 114 | AsyncBaseApi: imports | 2.9 |  | 0.625 |
| walker |  | 1191 | 19 | headings outline in py3xui/client/README.md |  |  | 0.625 |
| walker |  | 1209 | 18 | python decl doc at py3xui/client/client.py:7 |  |  | 0.625 |
| walker |  | 1234 | 25 | python decl names surface in py3xui/inbound/inbound.py |  |  | 0.625 |
| walker |  | 1234 | 0 | python decl at py3xui/inbound/inbound.py:15 |  |  | 0.625 |
| walker |  | 1234 | 0 | python decl at py3xui/inbound/inbound.py:38 |  |  | 0.625 |
| walker |  | 1252 | 18 | python decl doc at py3xui/inbound/inbound.py:15 |  |  | 0.625 |
| ns | 1297 |  | 123 | AsyncClientApi: method locations | 2.10 |  | 0.601 |
| walker |  | 1331 | 79 | README headline in py3xui/async_api/README.md |  |  | 0.601 |
| walker |  | 1371 | 40 | python method sigs in py3xui/inbound/inbound.py |  |  | 0.601 |
| walker |  | 1371 | 0 | python method at py3xui/inbound/inbound.py:114 |  |  | 0.601 |
| ns | 1382 |  | 85 | AsyncInboundApi: method locations | 2.11 |  | 0.586 |
| walker |  | 1440 | 69 | [package] in pyproject.toml |  |  | 0.587 |
| walker |  | 1465 | 25 | listing of 'tests' |  |  | 0.614 |
| walker |  | 1478 | 13 | python decl names surface in py3xui/async_api/async_api_base.py |  |  | 0.614 |
| walker |  | 1478 | 0 | python decl at py3xui/async_api/async_api_base.py:16 |  |  | 0.614 |
| walker |  | 1492 | 14 | python decl names surface in py3xui/api/api_client.py |  |  | 0.614 |
| walker |  | 1492 | 0 | python decl at py3xui/api/api_client.py:13 |  |  | 0.614 |
| walker |  | 1506 | 14 | python decl names surface in py3xui/api/api_database.py |  |  | 0.614 |
| walker |  | 1506 | 0 | python decl at py3xui/api/api_database.py:7 |  |  | 0.614 |
| ns | 1512 |  | 130 | AsyncApi + AsyncDatabaseApi + AsyncServerApi: method locations | 2.12 |  | 0.591 |
| walker |  | 1520 | 14 | python method sigs in py3xui/api/api_database.py |  |  | 0.591 |
| walker |  | 1520 | 0 | python method at py3xui/api/api_database.py:32 |  |  | 0.591 |
| walker |  | 1534 | 14 | python decl names surface in py3xui/api/api_server.py |  |  | 0.591 |
| walker |  | 1534 | 0 | python decl at py3xui/api/api_server.py:7 |  |  | 0.591 |
| walker |  | 1549 | 15 | python decl names surface in py3xui/api/api_inbound.py |  |  | 0.591 |
| walker |  | 1549 | 0 | python decl at py3xui/api/api_inbound.py:9 |  |  | 0.591 |
| walker |  | 1564 | 15 | python decl names surface in py3xui/inbound/bases.py |  |  | 0.591 |
| walker |  | 1564 | 0 | python decl at py3xui/inbound/bases.py:9 |  |  | 0.591 |
| walker |  | 1575 | 11 | python method sigs in py3xui/inbound/bases.py |  |  | 0.591 |
| ns | 1576 |  | 64 | .flake8 (full) | 2.13 |  | 0.581 |
| ns | 1597 |  | 21 | .pylintrc (full) | 2.14 |  | 0.578 |
| ns | 1993 |  | 396 | pyproject.toml (package metadata + deps) | 3.1 |  | 0.535 |
| walker |  | 2065 | 490 | README headline in README.md |  |  | 0.535 |
| ns | 2177 |  | 184 | Model classes: locations across bases/settings/sniffing/stream_settings/server + Inbound's own methods | 3.2 |  | 0.517 |
| walker |  | 2232 | 167 | headings outline in README.md |  |  | 0.517 |
| walker |  | 2394 | 162 | README.md section #0 |  |  | 0.519 |
| walker |  | 2437 | 43 | README.md section #22 |  |  | 0.519 |
| walker |  | 2449 | 12 | listing of '.vscode' |  |  | 0.531 |
| walker |  | 2466 | 17 | python decl names surface in py3xui/async_api/async_api_client.py |  |  | 0.531 |
| walker |  | 2466 | 0 | python decl at py3xui/async_api/async_api_client.py:12 |  |  | 0.531 |
| walker |  | 2483 | 17 | python decl names surface in py3xui/async_api/async_api_database.py |  |  | 0.531 |
| walker |  | 2483 | 0 | python decl at py3xui/async_api/async_api_database.py:7 |  |  | 0.531 |
| ns | 2495 |  | 318 | tests/test_api.py: test-function + _prepare_inbound locations | 3.3 |  | 0.499 |
| walker |  | 2498 | 15 | python method sigs in py3xui/async_api/async_api_database.py |  |  | 0.499 |
| walker |  | 2498 | 0 | python method at py3xui/async_api/async_api_database.py:32 |  |  | 0.499 |
| walker |  | 2515 | 17 | python decl names surface in py3xui/async_api/async_api_inbound.py |  |  | 0.499 |
| walker |  | 2515 | 0 | python decl at py3xui/async_api/async_api_inbound.py:11 |  |  | 0.499 |
| walker |  | 2532 | 17 | python decl names surface in py3xui/async_api/async_api_server.py |  |  | 0.499 |
| walker |  | 2532 | 0 | python decl at py3xui/async_api/async_api_server.py:8 |  |  | 0.499 |
| walker |  | 2548 | 16 | python class body at py3xui/async_api/async_api_server.py:8 |  |  | 0.499 |
| walker |  | 2567 | 19 | python decl doc at py3xui/inbound/bases.py:9 |  |  | 0.499 |
| walker |  | 2588 | 21 | python decl doc at py3xui/api/api_client.py:13 |  |  | 0.499 |
| walker |  | 2609 | 21 | python decl doc at py3xui/api/api_database.py:7 |  |  | 0.499 |
| walker |  | 2630 | 21 | python decl doc at py3xui/api/api_server.py:7 |  |  | 0.499 |
| walker |  | 2651 | 21 | python decl doc at py3xui/async_api/async_api_database.py:7 |  |  | 0.499 |
| ns | 2680 |  | 185 | tests/test_api.py: _prepare_inbound helper | 3.4 | 3.3 | 0.478 |
| walker |  | 2689 | 38 | python method at py3xui/inbound/inbound.py:88 |  |  | 0.478 |
| walker |  | 2820 | 131 | manifest config in pyproject.toml |  |  | 0.510 |
| walker |  | 2842 | 22 | python decl doc at py3xui/api/api_inbound.py:9 |  |  | 0.510 |
| walker |  | 2864 | 22 | python decl doc at py3xui/async_api/async_api_base.py:16 |  |  | 0.510 |
| walker |  | 2886 | 22 | python decl doc at py3xui/async_api/async_api_client.py:12 |  |  | 0.510 |
| walker |  | 2908 | 22 | python decl doc at py3xui/async_api/async_api_inbound.py:11 |  |  | 0.510 |
| walker |  | 2950 | 42 | python decl doc at py3xui/api/api.py:13 |  |  | 0.510 |
| walker |  | 2992 | 42 | python decl doc at py3xui/async_api/async_api.py:18 |  |  | 0.510 |
| walker |  | 3014 | 22 | python decl names surface in py3xui/api/api_base.py |  |  | 0.510 |
| walker |  | 3014 | 0 | python decl at py3xui/api/api_base.py:15 |  |  | 0.510 |
| walker |  | 3014 | 0 | python decl at py3xui/api/api_base.py:28 |  |  | 0.510 |
| walker |  | 3032 | 18 | python decl doc at py3xui/api/api_base.py:15 |  |  | 0.512 |
| walker |  | 3053 | 21 | python decl doc at py3xui/api/api_base.py:28 |  |  | 0.512 |
| ns | 3094 |  | 414 | README lede: concept, breaking-change warning, supported versions | 3.5 |  | 0.520 |
| ns | 3302 |  | 208 | demo.py: server status + db backup section | 3.6 |  | 0.505 |
| walker |  | 3401 | 348 | python decl names surface in demo.py |  |  | 0.505 |
| walker |  | 3401 | 0 | python decl at demo.py:70 |  |  | 0.505 |
| walker |  | 3419 | 18 | listing of '.github' |  |  | 0.518 |
| walker |  | 3442 | 23 | listing of '.github/workflows' |  |  | 0.534 |
| walker |  | 3466 | 24 | python decl names surface in py3xui/inbound/settings.py |  |  | 0.535 |
| walker |  | 3466 | 0 | python decl at py3xui/inbound/settings.py:9 |  |  | 0.535 |
| walker |  | 3466 | 0 | python decl at py3xui/inbound/settings.py:17 |  |  | 0.535 |
| walker |  | 3484 | 18 | python decl doc at py3xui/inbound/settings.py:9 |  |  | 0.535 |
| walker |  | 3519 | 35 | python class body at py3xui/inbound/settings.py:9 |  |  | 0.535 |
| walker |  | 3554 | 35 | python class body at py3xui/inbound/settings.py:17 |  |  | 0.535 |
| walker |  | 3580 | 26 | python decl names surface in py3xui/inbound/stream_settings.py |  |  | 0.537 |
| walker |  | 3580 | 0 | python decl at py3xui/inbound/stream_settings.py:9 |  |  | 0.537 |
| walker |  | 3580 | 0 | python decl at py3xui/inbound/stream_settings.py:25 |  |  | 0.537 |
| walker |  | 3598 | 18 | python decl doc at py3xui/inbound/stream_settings.py:9 |  |  | 0.537 |
| ns | 3613 |  | 311 | LICENSE.md (full) | 3.7 |  | 0.517 |
| walker |  | 3777 | 179 | python class body at py3xui/inbound/inbound.py:15 |  |  | 0.518 |
| walker |  | 3875 | 98 | headings outline in py3xui/server/README.md |  |  | 0.518 |
| walker |  | 3897 | 22 | README.md section #2 |  |  | 0.518 |
| walker |  | 3928 | 31 | python decl doc at py3xui/async_api/async_api_server.py:8 |  |  | 0.518 |
| ns | 3932 |  | 319 | utils/env.py: parse_env | 4.1 |  | 0.497 |
| walker |  | 3975 | 47 | python imports in demo.py |  |  | 0.497 |
| ns | 4142 |  | 210 | BaseApi: property/method locations | 4.2 |  | 0.482 |
| walker |  | 4200 | 225 | python class body at py3xui/client/client.py:7 |  |  | 0.482 |
| walker |  | 4254 | 54 | python method sigs in py3xui/api/api_server.py |  |  | 0.490 |
| walker |  | 4254 | 0 | python method at py3xui/api/api_server.py:35 |  |  | 0.490 |
| walker |  | 4254 | 0 | python method at py3xui/api/api_server.py:67 |  |  | 0.490 |
| walker |  | 4254 | 0 | python method at py3xui/api/api_server.py:97 |  |  | 0.490 |
| walker |  | 4282 | 28 | python decl names surface in py3xui/inbound/sniffing.py |  |  | 0.492 |
| walker |  | 4282 | 0 | python decl at py3xui/inbound/sniffing.py:9 |  |  | 0.492 |
| walker |  | 4282 | 0 | python decl at py3xui/inbound/sniffing.py:20 |  |  | 0.492 |
| walker |  | 4300 | 18 | python decl doc at py3xui/inbound/sniffing.py:9 |  |  | 0.492 |
| walker |  | 4355 | 55 | python method sigs in py3xui/async_api/async_api_server.py |  |  | 0.496 |
| walker |  | 4355 | 0 | python method at py3xui/async_api/async_api_server.py:44 |  |  | 0.496 |
| walker |  | 4355 | 0 | python method at py3xui/async_api/async_api_server.py:76 |  |  | 0.496 |
| walker |  | 4355 | 0 | python method at py3xui/async_api/async_api_server.py:105 |  |  | 0.496 |
| ns | 4457 |  | 315 | BaseApi.login body | 4.3 | 4.2 | 0.478 |
| walker |  | 4464 | 109 | python decl names surface in py3xui/server/server.py |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:7 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:43 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:54 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:67 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:78 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:89 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:100 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:113 |  |  | 0.505 |
| walker |  | 4464 | 0 | python decl at py3xui/server/server.py:125 |  |  | 0.505 |
| walker |  | 4482 | 18 | python class body at py3xui/server/server.py:43 |  |  | 0.505 |
| walker |  | 4500 | 18 | python class body at py3xui/server/server.py:67 |  |  | 0.505 |
| walker |  | 4518 | 18 | python class body at py3xui/server/server.py:78 |  |  | 0.505 |
| walker |  | 4538 | 20 | python class body at py3xui/server/server.py:89 |  |  | 0.505 |
| walker |  | 4554 | 16 | python decl doc at py3xui/server/server.py:7 |  |  | 0.505 |
| walker |  | 4581 | 27 | python class body at py3xui/server/server.py:100 |  |  | 0.505 |
| walker |  | 4603 | 22 | python decl doc at py3xui/server/server.py:113 |  |  | 0.505 |
| ns | 4635 |  | 178 | ServerApi.get_status body | 4.4 | 2.5 | 0.494 |
| walker |  | 4646 | 43 | python class body at py3xui/server/server.py:54 |  |  | 0.494 |
| walker |  | 4698 | 52 | python decl doc at py3xui/server/server.py:78 |  |  | 0.494 |
| walker |  | 4752 | 54 | python decl doc at py3xui/server/server.py:67 |  |  | 0.494 |
| walker |  | 4809 | 57 | python decl doc at py3xui/server/server.py:43 |  |  | 0.494 |
| walker |  | 4868 | 59 | python decl doc at py3xui/server/server.py:89 |  |  | 0.494 |
| ns | 4897 |  | 262 | InboundApi.get_list body | 4.5 | 2.4 | 0.480 |
| walker |  | 4988 | 120 | python method sigs in py3xui/api/api.py |  |  | 0.494 |
| walker |  | 4988 | 0 | python method at py3xui/api/api.py:189 |  |  | 0.494 |
| walker |  | 4996 | 8 | python method at py3xui/api/api.py:93 |  |  | 0.494 |
| walker |  | 5004 | 8 | python method at py3xui/api/api.py:115 |  |  | 0.494 |
| walker |  | 5013 | 9 | python method at py3xui/api/api.py:102 |  |  | 0.494 |
| walker |  | 5023 | 10 | python method at py3xui/api/api.py:124 |  |  | 0.494 |
| walker |  | 5033 | 10 | python method body at py3xui/api/api.py:93 body 100 |  |  | 0.494 |
| walker |  | 5044 | 11 | python method body at py3xui/api/api.py:115 body 122 |  |  | 0.494 |
| walker |  | 5151 | 107 | python decl doc at demo.py:70 |  |  | 0.494 |
| ns | 5227 |  | 330 | ClientApi.add body | 4.6 | 2.6 | 0.480 |
| walker |  | 5272 | 121 | python method sigs in py3xui/async_api/async_api.py |  |  | 0.499 |
| walker |  | 5272 | 0 | python method at py3xui/async_api/async_api.py:194 |  |  | 0.499 |
| walker |  | 5280 | 8 | python method at py3xui/async_api/async_api.py:98 |  |  | 0.499 |
| walker |  | 5288 | 8 | python method at py3xui/async_api/async_api.py:120 |  |  | 0.499 |
| walker |  | 5297 | 9 | python method at py3xui/async_api/async_api.py:107 |  |  | 0.499 |
| walker |  | 5307 | 10 | python method at py3xui/async_api/async_api.py:129 |  |  | 0.499 |
| walker |  | 5317 | 10 | python method body at py3xui/async_api/async_api.py:98 body 105 |  |  | 0.499 |
| walker |  | 5328 | 11 | python method body at py3xui/async_api/async_api.py:120 body 127 |  |  | 0.499 |
| ns | 5576 |  | 349 | ClientApi.get_by_email body | 4.7 | 2.6 | 0.483 |
| walker |  | 5675 | 347 | python class body at py3xui/server/server.py:7 |  |  | 0.483 |
| walker |  | 5991 | 316 | python class body at py3xui/inbound/inbound.py:38 |  |  | 0.484 |
| walker |  | 6074 | 83 | python class body at py3xui/server/server.py:113 |  |  | 0.484 |
| ns | 6115 |  | 539 | README Quick Start: install + instantiate sync/async | 4.8 |  | 0.463 |
| walker |  | 6140 | 66 | python decl doc at py3xui/server/server.py:100 |  |  | 0.463 |
| walker |  | 6216 | 76 | python decl doc at py3xui/server/server.py:54 |  |  | 0.463 |
| walker |  | 6273 | 57 | python class body at py3xui/inbound/sniffing.py:9 |  |  | 0.463 |
| walker |  | 6340 | 67 | python method at py3xui/api/api.py:139 |  |  | 0.463 |
| walker |  | 6483 | 143 | headings outline in py3xui/api/README.md |  |  | 0.463 |
| walker |  | 6551 | 68 | python method at py3xui/async_api/async_api.py:144 |  |  | 0.463 |
| ns | 6625 |  | 510 | Api.from_env body | 4.9 | 2.3 | 0.444 |
| walker |  | 6891 | 340 | python class body at py3xui/server/server.py:125 |  |  | 0.444 |
| walker |  | 6935 | 44 | python method at py3xui/inbound/bases.py:12 |  |  | 0.433 |
| ns | 6935 |  | 310 | AsyncBaseApi._request_with_retry: the sync/async architecture delta | 4.10 |  | 0.433 |
| walker |  | 7086 | 151 | headings outline in py3xui/inbound/README.md |  |  | 0.433 |
| ns | 7092 |  | 157 | Api.login body | 4.11 | 2.3 | 0.428 |
| walker |  | 7151 | 65 | plaintext config dev/requirements.txt |  |  | 0.428 |
| walker |  | 7561 | 410 | python class body at py3xui/client/client.py:36 |  |  | 0.429 |
| walker |  | 7637 | 76 | python class body at py3xui/api/api_base.py:15 |  |  | 0.443 |
| walker |  | 7800 | 163 | headings outline in py3xui/async_api/README.md |  |  | 0.443 |
| walker |  | 7847 | 47 | python method doc at py3xui/api/api.py:93 |  |  | 0.443 |
| ns | 7882 |  | 790 | Client model (full) | 5.1 |  | 0.441 |
| walker |  | 7894 | 47 | python method doc at py3xui/async_api/async_api.py:98 |  |  | 0.441 |
| walker |  | 7943 | 49 | python method doc at py3xui/api/api.py:102 |  |  | 0.441 |
| walker |  | 7992 | 49 | python method doc at py3xui/async_api/async_api.py:107 |  |  | 0.441 |
| walker |  | 8034 | 42 | python imports in py3xui/server/server.py |  |  | 0.441 |
| walker |  | 8077 | 43 | python imports in py3xui/client/client.py |  |  | 0.441 |
| walker |  | 8122 | 45 | README.md section #7 |  |  | 0.441 |
| walker |  | 8243 | 121 | python class body at py3xui/inbound/stream_settings.py:9 |  |  | 0.441 |
| walker |  | 8306 | 63 | py3xui/server/README.md section #1 |  |  | 0.441 |
| walker |  | 8360 | 54 | python method doc at py3xui/async_api/async_api.py:120 |  |  | 0.441 |
| walker |  | 8717 | 357 | python method sigs in py3xui/api/api_base.py |  |  | 0.468 |
| walker |  | 8717 | 0 | python method at py3xui/api/api_base.py:168 |  |  | 0.468 |
| walker |  | 8717 | 0 | python method at py3xui/api/api_base.py:194 |  |  | 0.468 |
| walker |  | 8717 | 0 | python method at py3xui/api/api_base.py:220 |  |  | 0.468 |
| walker |  | 8717 | 0 | python method at py3xui/api/api_base.py:236 |  |  | 0.468 |
| walker |  | 8717 | 0 | python method at py3xui/api/api_base.py:333 |  |  | 0.468 |
| walker |  | 8725 | 8 | python method at py3xui/api/api_base.py:80 |  |  | 0.468 |
| walker |  | 8733 | 8 | python method at py3xui/api/api_base.py:88 |  |  | 0.468 |
| walker |  | 8741 | 8 | python method at py3xui/api/api_base.py:96 |  |  | 0.468 |
| walker |  | 8749 | 8 | python method at py3xui/api/api_base.py:104 |  |  | 0.468 |
| walker |  | 8757 | 8 | python method at py3xui/api/api_base.py:112 |  |  | 0.468 |
| ns | 8761 |  | 879 | Inbound model: fields (full) | 5.2 |  | 0.466 |
| walker |  | 8765 | 8 | python method at py3xui/api/api_base.py:120 |  |  | 0.466 |
| walker |  | 8773 | 8 | python method at py3xui/api/api_base.py:136 |  |  | 0.466 |
| walker |  | 8781 | 8 | python method at py3xui/api/api_base.py:152 |  |  | 0.466 |
| walker |  | 8789 | 8 | python method at py3xui/api/api_base.py:210 |  |  | 0.466 |
| walker |  | 8798 | 9 | python method at py3xui/api/api_base.py:144 |  |  | 0.466 |
| walker |  | 8808 | 10 | python method at py3xui/api/api_base.py:160 |  |  | 0.466 |
| walker |  | 8819 | 11 | python method at py3xui/api/api_base.py:128 |  |  | 0.466 |
| walker |  | 8829 | 10 | python method body at py3xui/api/api_base.py:80 body 86 |  |  | 0.466 |
| walker |  | 8839 | 10 | python method body at py3xui/api/api_base.py:88 body 94 |  |  | 0.466 |
| walker |  | 8849 | 10 | python method body at py3xui/api/api_base.py:96 body 102 |  |  | 0.466 |
| walker |  | 8859 | 10 | python method body at py3xui/api/api_base.py:136 body 142 |  |  | 0.466 |
| walker |  | 8870 | 11 | python method body at py3xui/api/api_base.py:144 body 150 |  |  | 0.466 |
| walker |  | 8881 | 11 | python method body at py3xui/api/api_base.py:152 body 158 |  |  | 0.466 |
| walker |  | 8893 | 12 | python method body at py3xui/api/api_base.py:104 body 110 |  |  | 0.466 |
| walker |  | 8905 | 12 | python method body at py3xui/api/api_base.py:112 body 118 |  |  | 0.466 |
| walker |  | 8917 | 12 | python method body at py3xui/api/api_base.py:120 body 126 |  |  | 0.466 |
| walker |  | 8929 | 12 | python method body at py3xui/api/api_base.py:160 body 166 |  |  | 0.466 |
| walker |  | 8942 | 13 | python method body at py3xui/api/api_base.py:128 body 134 |  |  | 0.466 |
| ns | 9002 |  | 241 | tests/responses/get_server_status.json (excerpt) | 5.3 |  | 0.459 |
| walker |  | 9006 | 64 | py3xui/client/README.md section #1 |  |  | 0.459 |
| walker |  | 9338 | 332 | python class body at py3xui/inbound/stream_settings.py:25 |  |  | 0.459 |
| ns | 9529 |  | 527 | BaseApi._request_with_retry body (core logic) | 5.4 | 4.2 | 0.445 |
| walker |  | 9704 | 366 | python method sigs in py3xui/async_api/async_api_base.py |  |  | 0.445 |
| walker |  | 9704 | 0 | python method at py3xui/async_api/async_api_base.py:156 |  |  | 0.445 |
| walker |  | 9704 | 0 | python method at py3xui/async_api/async_api_base.py:239 |  |  | 0.445 |
| walker |  | 9704 | 0 | python method at py3xui/async_api/async_api_base.py:265 |  |  | 0.445 |
| walker |  | 9704 | 0 | python method at py3xui/async_api/async_api_base.py:292 |  |  | 0.445 |
| walker |  | 9704 | 0 | python method at py3xui/async_api/async_api_base.py:328 |  |  | 0.445 |
| walker |  | 9712 | 8 | python method at py3xui/async_api/async_api_base.py:68 |  |  | 0.445 |
| walker |  | 9720 | 8 | python method at py3xui/async_api/async_api_base.py:76 |  |  | 0.445 |
| walker |  | 9728 | 8 | python method at py3xui/async_api/async_api_base.py:84 |  |  | 0.445 |
| walker |  | 9736 | 8 | python method at py3xui/async_api/async_api_base.py:92 |  |  | 0.445 |
| walker |  | 9744 | 8 | python method at py3xui/async_api/async_api_base.py:100 |  |  | 0.445 |
| walker |  | 9752 | 8 | python method at py3xui/async_api/async_api_base.py:108 |  |  | 0.445 |
| walker |  | 9760 | 8 | python method at py3xui/async_api/async_api_base.py:124 |  |  | 0.445 |
| walker |  | 9768 | 8 | python method at py3xui/async_api/async_api_base.py:140 |  |  | 0.445 |
| walker |  | 9776 | 8 | python method at py3xui/async_api/async_api_base.py:281 |  |  | 0.445 |
| walker |  | 9785 | 9 | python method at py3xui/async_api/async_api_base.py:132 |  |  | 0.445 |
| walker |  | 9795 | 10 | python method at py3xui/async_api/async_api_base.py:148 |  |  | 0.445 |
| walker |  | 9806 | 11 | python method at py3xui/async_api/async_api_base.py:116 |  |  | 0.445 |
| walker |  | 9816 | 10 | python method body at py3xui/async_api/async_api_base.py:68 body 74 |  |  | 0.445 |
| walker |  | 9826 | 10 | python method body at py3xui/async_api/async_api_base.py:76 body 82 |  |  | 0.445 |
| walker |  | 9836 | 10 | python method body at py3xui/async_api/async_api_base.py:84 body 90 |  |  | 0.445 |
| walker |  | 9846 | 10 | python method body at py3xui/async_api/async_api_base.py:124 body 130 |  |  | 0.445 |
| walker |  | 9857 | 11 | python method body at py3xui/async_api/async_api_base.py:132 body 138 |  |  | 0.445 |
| walker |  | 9868 | 11 | python method body at py3xui/async_api/async_api_base.py:140 body 146 |  |  | 0.445 |
| walker |  | 9880 | 12 | python method body at py3xui/async_api/async_api_base.py:92 body 98 |  |  | 0.445 |
| walker |  | 9892 | 12 | python method body at py3xui/async_api/async_api_base.py:100 body 106 |  |  | 0.445 |
| walker |  | 9904 | 12 | python method body at py3xui/async_api/async_api_base.py:108 body 114 |  |  | 0.445 |
| walker |  | 9916 | 12 | python method body at py3xui/async_api/async_api_base.py:148 body 154 |  |  | 0.445 |
| walker |  | 9929 | 13 | python method body at py3xui/async_api/async_api_base.py:116 body 122 |  |  | 0.445 |
| walker |  | 9985 | 56 | python method doc at py3xui/inbound/inbound.py:114 |  |  | 0.445 |
| ns | 9999 |  | 470 | .github/workflows/checks.yml (full) | 5.5 |  | 0.431 |

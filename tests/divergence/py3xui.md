Score(3000)=0.533 I=0.816 C=0.348 ns_rows≤3K=31/58 (reached=14 partial=0 missing=17)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 19 |  | 19 | README lede — one-line domain statement | 1.1 |  | 0.000 |
| walker |  | 56 | 56 | listing of '.' |  |  | 0.000 |
| walker |  | 81 | 25 | listing of 'py3xui' |  |  | 0.000 |
| ns | 100 |  | 81 | Top-level package layout | 1.2 |  | 0.837 |
| walker |  | 175 | 94 | python imports in py3xui/__init__.py |  |  | 0.871 |
| walker |  | 189 | 14 | listing of 'py3xui/client' |  |  | 0.872 |
| ns | 194 |  | 94 | Public re-exports from py3xui/__init__.py | 1.3 |  | 0.850 |
| walker |  | 203 | 14 | listing of 'py3xui/server' |  |  | 0.852 |
| walker |  | 217 | 14 | listing of 'py3xui/utils' |  |  | 0.854 |
| ns | 241 |  | 47 | pyproject.toml — name, version, description | 1.4 |  | 0.804 |
| walker |  | 245 | 28 | python imports in py3xui/client/__init__.py |  |  | 0.804 |
| walker |  | 260 | 15 | python decl names surface in py3xui/utils/__init__.py |  |  | 0.804 |
| walker |  | 292 | 32 | python imports in py3xui/server/__init__.py |  |  | 0.804 |
| ns | 298 |  | 57 | pyproject.toml — runtime dependencies | 1.5 |  | 0.735 |
| walker |  | 325 | 33 | python imports in py3xui/utils/__init__.py |  |  | 0.735 |
| ns | 378 |  | 80 | Sync sub-API exports | 1.6 |  | 0.679 |
| walker |  | 382 | 57 | [dependencies] in pyproject.toml |  |  | 0.755 |
| walker |  | 414 | 32 | listing of 'py3xui/inbound' |  |  | 0.764 |
| walker |  | 464 | 50 | README headline in py3xui/utils/README.md |  |  | 0.764 |
| ns | 469 |  | 91 | Async sub-API exports | 1.7 |  | 0.713 |
| walker |  | 516 | 52 | README headline in py3xui/inbound/README.md |  |  | 0.713 |
| walker |  | 556 | 40 | listing of 'py3xui/api' |  |  | 0.732 |
| walker |  | 602 | 46 | listing of 'py3xui/async_api' |  |  | 0.757 |
| ns | 629 |  | 160 | Inner-package directory listings | 1.8 |  | 0.775 |
| ns | 678 |  | 49 | Api class signature + lede | 2.1 |  | 0.758 |
| walker |  | 682 | 80 | python imports in py3xui/api/__init__.py |  |  | 0.800 |
| ns | 728 |  | 50 | AsyncApi class signature + lede | 2.2 |  | 0.783 |
| walker |  | 764 | 82 | python imports in py3xui/inbound/__init__.py |  |  | 0.783 |
| ns | 832 |  | 104 | Api constructor signature | 2.3 | 2.1 | 0.738 |
| walker |  | 855 | 91 | python imports in py3xui/async_api/__init__.py |  |  | 0.775 |
| walker |  | 882 | 27 | listing of 'dev' |  |  | 0.775 |
| walker |  | 936 | 54 | README headline in py3xui/api/README.md |  |  | 0.734 |
| ns | 936 |  | 104 | AsyncApi constructor signature | 2.4 | 2.2 | 0.734 |
| walker |  | 991 | 55 | README headline in py3xui/server/README.md |  |  | 0.734 |
| walker |  | 1047 | 56 | README headline in py3xui/client/README.md |  |  | 0.734 |
| walker |  | 1068 | 21 | headings outline in py3xui/client/README.md |  |  | 0.734 |
| walker |  | 1077 | 9 | python decl names surface in py3xui/api/api.py |  |  | 0.734 |
| walker |  | 1077 | 0 | python decl at py3xui/api/api.py:13 |  |  | 0.734 |
| ns | 1142 |  | 206 | Api sub-client wiring (client/inbound/database/server attrs) | 2.5 | 2.3 | 0.674 |
| walker |  | 1144 | 67 | README headline in py3xui/async_api/README.md |  |  | 0.674 |
| walker |  | 1154 | 10 | python decl names surface in py3xui/async_api/async_api.py |  |  | 0.674 |
| walker |  | 1154 | 0 | python decl at py3xui/async_api/async_api.py:18 |  |  | 0.674 |
| walker |  | 1165 | 11 | python decl names surface in py3xui/async_api/async_api_base.py |  |  | 0.674 |
| walker |  | 1165 | 0 | python decl at py3xui/async_api/async_api_base.py:16 |  |  | 0.674 |
| walker |  | 1177 | 12 | python decl names surface in py3xui/api/api_client.py |  |  | 0.674 |
| walker |  | 1177 | 0 | python decl at py3xui/api/api_client.py:13 |  |  | 0.674 |
| walker |  | 1189 | 12 | python decl names surface in py3xui/api/api_database.py |  |  | 0.674 |
| walker |  | 1189 | 0 | python decl at py3xui/api/api_database.py:7 |  |  | 0.674 |
| walker |  | 1203 | 14 | python method sigs in py3xui/api/api_database.py |  |  | 0.675 |
| walker |  | 1203 | 0 | python method at py3xui/api/api_database.py:32 |  |  | 0.675 |
| walker |  | 1215 | 12 | python decl names surface in py3xui/api/api_server.py |  |  | 0.675 |
| walker |  | 1215 | 0 | python decl at py3xui/api/api_server.py:7 |  |  | 0.675 |
| walker |  | 1280 | 65 | [package] in pyproject.toml |  |  | 0.697 |
| walker |  | 1293 | 13 | python decl names surface in py3xui/api/api_inbound.py |  |  | 0.697 |
| walker |  | 1293 | 0 | python decl at py3xui/api/api_inbound.py:9 |  |  | 0.697 |
| walker |  | 1306 | 13 | python decl names surface in py3xui/inbound/bases.py |  |  | 0.697 |
| walker |  | 1306 | 0 | python decl at py3xui/inbound/bases.py:9 |  |  | 0.697 |
| walker |  | 1320 | 14 | python method sigs in py3xui/inbound/bases.py |  |  | 0.697 |
| walker |  | 1345 | 25 | listing of 'tests' |  |  | 0.697 |
| ns | 1356 |  | 214 | AsyncApi sub-client wiring | 2.6 | 2.4 | 0.644 |
| walker |  | 1362 | 17 | python decl doc at py3xui/inbound/bases.py:9 |  |  | 0.644 |
| walker |  | 1377 | 15 | python decl names surface in py3xui/async_api/async_api_client.py |  |  | 0.644 |
| walker |  | 1377 | 0 | python decl at py3xui/async_api/async_api_client.py:12 |  |  | 0.644 |
| walker |  | 1392 | 15 | python decl names surface in py3xui/async_api/async_api_database.py |  |  | 0.644 |
| walker |  | 1392 | 0 | python decl at py3xui/async_api/async_api_database.py:7 |  |  | 0.644 |
| walker |  | 1407 | 15 | python method sigs in py3xui/async_api/async_api_database.py |  |  | 0.644 |
| walker |  | 1407 | 0 | python method at py3xui/async_api/async_api_database.py:32 |  |  | 0.644 |
| walker |  | 1422 | 15 | python decl names surface in py3xui/async_api/async_api_inbound.py |  |  | 0.644 |
| walker |  | 1422 | 0 | python decl at py3xui/async_api/async_api_inbound.py:11 |  |  | 0.644 |
| walker |  | 1437 | 15 | python decl names surface in py3xui/async_api/async_api_server.py |  |  | 0.644 |
| walker |  | 1437 | 0 | python decl at py3xui/async_api/async_api_server.py:8 |  |  | 0.644 |
| walker |  | 1451 | 14 | python class body at py3xui/async_api/async_api_server.py:8 |  |  | 0.644 |
| ns | 1526 |  | 170 | Facade public methods (Api.from_env, Api.login) | 2.7 | 2.1 | 0.608 |
| ns | 1698 |  | 172 | AsyncApi public methods | 2.8 | 2.2 | 0.577 |
| ns | 1789 |  | 91 | ClientApi method roster (sync, names only) | 3.1 |  | 0.557 |
| ns | 1851 |  | 62 | InboundApi method roster (sync, names only) | 3.2 |  | 0.545 |
| ns | 1880 |  | 29 | ServerApi method roster (sync, names only) | 3.3 |  | 0.539 |
| ns | 1888 |  | 8 | DatabaseApi method roster (sync, names only) | 3.4 |  | 0.541 |
| walker |  | 1933 | 482 | README headline in README.md |  |  | 0.609 |
| ns | 1989 |  | 101 | AsyncClientApi method roster (async, names only) | 3.5 |  | 0.591 |
| ns | 2058 |  | 69 | AsyncInboundApi method roster (async, names only) | 3.6 |  | 0.579 |
| ns | 2099 |  | 41 | AsyncServerApi + AsyncDatabaseApi method roster | 3.7 |  | 0.573 |
| walker |  | 2100 | 167 | headings outline in README.md |  |  | 0.573 |
| ns | 2203 |  | 104 | ClientApi full signatures (sync) | 3.8 | 3.1 | 0.557 |
| walker |  | 2254 | 154 | README.md section #0 |  |  | 0.557 |
| ns | 2264 |  | 61 | InboundApi full signatures (sync) | 3.9 | 3.2 | 0.547 |
| ns | 2283 |  | 19 | ServerApi full signatures (sync) | 3.10 | 3.3 | 0.543 |
| ns | 2287 |  | 4 | DatabaseApi full signature (sync) | 3.11 | 3.4 | 0.545 |
| walker |  | 2297 | 43 | README.md section #22 |  |  | 0.545 |
| walker |  | 2317 | 20 | python decl names surface in py3xui/api/api_base.py |  |  | 0.545 |
| walker |  | 2317 | 0 | python decl at py3xui/api/api_base.py:15 |  |  | 0.545 |
| walker |  | 2317 | 0 | python decl at py3xui/api/api_base.py:28 |  |  | 0.545 |
| walker |  | 2333 | 16 | python decl doc at py3xui/api/api_base.py:15 |  |  | 0.545 |
| walker |  | 2354 | 21 | python decl names surface in py3xui/client/client.py |  |  | 0.545 |
| walker |  | 2354 | 0 | python decl at py3xui/client/client.py:7 |  |  | 0.545 |
| walker |  | 2354 | 0 | python decl at py3xui/client/client.py:36 |  |  | 0.545 |
| walker |  | 2370 | 16 | python decl doc at py3xui/client/client.py:7 |  |  | 0.545 |
| walker |  | 2392 | 22 | python decl names surface in py3xui/inbound/settings.py |  |  | 0.545 |
| walker |  | 2392 | 0 | python decl at py3xui/inbound/settings.py:9 |  |  | 0.545 |
| walker |  | 2392 | 0 | python decl at py3xui/inbound/settings.py:17 |  |  | 0.545 |
| walker |  | 2408 | 16 | python decl doc at py3xui/inbound/settings.py:9 |  |  | 0.545 |
| ns | 2429 |  | 142 | Client model — fields list (field names only) | 4.1 |  | 0.520 |
| walker |  | 2443 | 35 | python class body at py3xui/inbound/settings.py:17 |  |  | 0.521 |
| walker |  | 2483 | 40 | python imports in demo.py |  |  | 0.521 |
| walker |  | 2506 | 23 | python decl names surface in py3xui/inbound/inbound.py |  |  | 0.521 |
| walker |  | 2506 | 0 | python decl at py3xui/inbound/inbound.py:15 |  |  | 0.521 |
| walker |  | 2506 | 0 | python decl at py3xui/inbound/inbound.py:38 |  |  | 0.521 |
| walker |  | 2522 | 16 | python decl doc at py3xui/inbound/inbound.py:15 |  |  | 0.521 |
| walker |  | 2554 | 32 | python method sigs in py3xui/inbound/inbound.py |  |  | 0.521 |
| walker |  | 2554 | 0 | python method at py3xui/inbound/inbound.py:114 |  |  | 0.521 |
| ns | 2576 |  | 147 | Inbound model — fields list | 4.2 |  | 0.502 |
| walker |  | 2591 | 37 | python class body at py3xui/inbound/settings.py:9 |  |  | 0.502 |
| walker |  | 2615 | 24 | python decl names surface in py3xui/inbound/stream_settings.py |  |  | 0.503 |
| walker |  | 2615 | 0 | python decl at py3xui/inbound/stream_settings.py:9 |  |  | 0.503 |
| walker |  | 2615 | 0 | python decl at py3xui/inbound/stream_settings.py:25 |  |  | 0.503 |
| walker |  | 2631 | 16 | python decl doc at py3xui/inbound/stream_settings.py:9 |  |  | 0.503 |
| walker |  | 2657 | 26 | python decl names surface in py3xui/inbound/sniffing.py |  |  | 0.494 |
| walker |  | 2657 | 0 | python decl at py3xui/inbound/sniffing.py:9 |  |  | 0.494 |
| walker |  | 2657 | 0 | python decl at py3xui/inbound/sniffing.py:20 |  |  | 0.494 |
| ns | 2657 |  | 81 | Server, MemoryInfo, XRayInfo, NetworkIO, NetworkTraffic, PublicIP, AppStats, RealityKeyPair class headers | 4.3 |  | 0.494 |
| walker |  | 2673 | 16 | python decl doc at py3xui/inbound/sniffing.py:9 |  |  | 0.495 |
| walker |  | 2695 | 22 | README.md section #2 |  |  | 0.495 |
| ns | 2701 |  | 44 | Settings, Sniffing, StreamSettings, JsonStringModel class headers | 4.4 |  | 0.502 |
| walker |  | 2749 | 54 | python method sigs in py3xui/api/api_server.py |  |  | 0.521 |
| walker |  | 2749 | 0 | python method at py3xui/api/api_server.py:35 |  |  | 0.521 |
| walker |  | 2749 | 0 | python method at py3xui/api/api_server.py:67 |  |  | 0.521 |
| walker |  | 2749 | 0 | python method at py3xui/api/api_server.py:97 |  |  | 0.521 |
| walker |  | 2849 | 100 | headings outline in py3xui/server/README.md |  |  | 0.521 |
| walker |  | 2906 | 57 | python method sigs in py3xui/async_api/async_api_server.py |  |  | 0.533 |
| walker |  | 2906 | 0 | python method at py3xui/async_api/async_api_server.py:44 |  |  | 0.533 |
| walker |  | 2906 | 0 | python method at py3xui/async_api/async_api_server.py:76 |  |  | 0.533 |
| walker |  | 2906 | 0 | python method at py3xui/async_api/async_api_server.py:105 |  |  | 0.533 |
| walker |  | 2955 | 49 | python class body at py3xui/inbound/sniffing.py:9 |  |  | 0.533 |
| walker |  | 2996 | 41 | python method at py3xui/inbound/bases.py:12 |  |  | 0.533 |
| ns | 3001 |  | 300 | StreamSettings full field list (network/security/tcp/kcp/reality/xtls/tls/xhttp) | 4.5 |  | 0.509 |
| walker |  | 3046 | 50 | py3xui/server/README.md section #1 |  |  | 0.509 |
| walker |  | 3090 | 44 | python method at py3xui/inbound/inbound.py:88 |  |  | 0.509 |
| walker |  | 3141 | 51 | py3xui/client/README.md section #1 |  |  | 0.509 |
| walker |  | 3284 | 143 | headings outline in py3xui/api/README.md |  |  | 0.509 |
| ns | 3309 |  | 308 | Server full field list | 4.6 | 4.3 | 0.494 |
| walker |  | 3435 | 151 | headings outline in py3xui/inbound/README.md |  |  | 0.494 |
| ns | 3455 |  | 146 | Settings + Sniffing full field lists | 4.7 | 4.4 | 0.487 |
| walker |  | 3598 | 163 | headings outline in py3xui/async_api/README.md |  |  | 0.487 |
| ns | 3639 |  | 184 | JsonStringModel preprocessing validator | 4.8 | 4.4 | 0.475 |
| ns | 3751 |  | 112 | RealityKeyPair full body | 4.9 | 4.3 | 0.467 |
| walker |  | 3776 | 178 | python decl names surface in demo.py |  |  | 0.467 |
| walker |  | 3852 | 76 | python class body at py3xui/api/api_base.py:15 |  |  | 0.468 |
| ns | 3863 |  | 112 | ApiFields constants (response keys + HTTP methods) | 5.1 |  | 0.480 |
| ns | 3892 |  | 29 | BaseApi class signature + summary | 5.2 |  | 0.479 |
| ns | 3923 |  | 31 | AsyncBaseApi class signature | 5.3 |  | 0.478 |
| walker |  | 3942 | 90 | python method sigs in py3xui/api/api.py |  |  | 0.484 |
| walker |  | 3942 | 0 | python method at py3xui/api/api.py:189 |  |  | 0.484 |
| walker |  | 3954 | 12 | python method at py3xui/api/api.py:93 |  |  | 0.484 |
| walker |  | 3967 | 13 | python method at py3xui/api/api.py:115 |  |  | 0.484 |
| walker |  | 3981 | 14 | python method at py3xui/api/api.py:102 |  |  | 0.487 |
| walker |  | 3998 | 17 | python method at py3xui/api/api.py:124 |  |  | 0.490 |
| walker |  | 4008 | 10 | python method body at py3xui/api/api.py:93 body 100 |  |  | 0.490 |
| walker |  | 4019 | 11 | python method body at py3xui/api/api.py:115 body 122 |  |  | 0.490 |
| ns | 4050 |  | 127 | BaseApi __init__ field set | 5.4 | 5.2 | 0.484 |
| walker |  | 4110 | 91 | python method sigs in py3xui/async_api/async_api.py |  |  | 0.490 |
| walker |  | 4110 | 0 | python method at py3xui/async_api/async_api.py:194 |  |  | 0.490 |
| walker |  | 4122 | 12 | python method at py3xui/async_api/async_api.py:98 |  |  | 0.490 |
| walker |  | 4135 | 13 | python method at py3xui/async_api/async_api.py:120 |  |  | 0.490 |
| walker |  | 4149 | 14 | python method at py3xui/async_api/async_api.py:107 |  |  | 0.492 |
| walker |  | 4166 | 17 | python method at py3xui/async_api/async_api.py:129 |  |  | 0.496 |
| ns | 4174 |  | 124 | BaseApi property names roster (host/username/password/use_tls_verify/custom_certificate_path/max_retries/session/cookie_name/cookies) | 5.5 | 5.2 | 0.489 |
| walker |  | 4176 | 10 | python method body at py3xui/async_api/async_api.py:98 body 105 |  |  | 0.489 |
| walker |  | 4187 | 11 | python method body at py3xui/async_api/async_api.py:120 body 127 |  |  | 0.489 |
| walker |  | 4230 | 43 | README.md section #7 |  |  | 0.489 |
| ns | 4391 |  | 217 | BaseApi method roster (login, _get_cookie, _check_response, _url, _request_with_retry, _post, _get) | 5.6 | 5.2 | 0.479 |
| walker |  | 4427 | 197 | py3xui/inbound/README.md section #0 |  |  | 0.479 |
| ns | 4507 |  | 116 | COOKIE_NAMES and cookies property | 5.7 | 5.5 | 0.472 |
| walker |  | 4581 | 154 | README.md section #1 |  |  | 0.472 |
| walker |  | 4707 | 126 | python decl names surface #1 in demo.py |  |  | 0.472 |
| walker |  | 4707 | 0 | python decl at demo.py:70 |  |  | 0.472 |
| walker |  | 4802 | 95 | python decl doc at demo.py:70 |  |  | 0.472 |
| walker |  | 4903 | 101 | python class body at py3xui/inbound/sniffing.py:20 |  |  | 0.486 |
| ns | 4968 |  | 461 | BaseApi.login body — sends credentials, captures cookie | 5.8 | 5.6 | 0.460 |
| walker |  | 4969 | 66 | python method at py3xui/api/api.py:139 |  |  | 0.480 |
| walker |  | 5036 | 67 | python method at py3xui/async_api/async_api.py:144 |  |  | 0.499 |
| walker |  | 5069 | 33 | python imports in py3xui/utils/env.py |  |  | 0.499 |
| walker |  | 5221 | 152 | README.md section #12 |  |  | 0.499 |
| walker |  | 5334 | 113 | python class body at py3xui/inbound/stream_settings.py:9 |  |  | 0.500 |
| walker |  | 5421 | 87 | python decl doc at py3xui/inbound/settings.py:17 |  |  | 0.500 |
| walker |  | 5558 | 137 | python method sigs in py3xui/api/api_inbound.py |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:40 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:71 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:111 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:159 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:190 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:223 |  |  | 0.527 |
| walker |  | 5558 | 0 | python method at py3xui/api/api_inbound.py:247 |  |  | 0.527 |
| walker |  | 5609 | 51 | README.md section #16 |  |  | 0.527 |
| walker |  | 5653 | 44 | python method doc at py3xui/api/api.py:93 |  |  | 0.527 |
| walker |  | 5697 | 44 | python method doc at py3xui/api/api.py:102 |  |  | 0.527 |
| walker |  | 5741 | 44 | python method doc at py3xui/async_api/async_api.py:98 |  |  | 0.527 |
| walker |  | 5785 | 44 | python method doc at py3xui/async_api/async_api.py:107 |  |  | 0.527 |
| ns | 5798 |  | 830 | _request_with_retry — TLS verify branch + retry/backoff body (sync) | 5.9 | 5.6 | 0.488 |
| walker |  | 5929 | 144 | python method sigs in py3xui/async_api/async_api_inbound.py |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:42 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:72 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:112 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:158 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:189 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:222 |  |  | 0.500 |
| walker |  | 5929 | 0 | python method at py3xui/async_api/async_api_inbound.py:246 |  |  | 0.500 |
| walker |  | 5966 | 37 | python imports in py3xui/server/server.py |  |  | 0.500 |
| walker |  | 6063 | 97 | python decl doc at py3xui/inbound/sniffing.py:20 |  |  | 0.500 |
| walker |  | 6101 | 38 | python imports in py3xui/client/client.py |  |  | 0.500 |
| walker |  | 6139 | 38 | python imports in py3xui/inbound/bases.py |  |  | 0.500 |
| walker |  | 6188 | 49 | python method doc at py3xui/inbound/inbound.py:114 |  |  | 0.500 |
| ns | 6238 |  | 440 | _post and _get bodies — login gate + delegation to retry | 5.10 | 5.6 | 0.481 |
| ns | 6395 |  | 157 | _check_response — success-field validation | 5.11 | 5.6 | 0.474 |
| walker |  | 6479 | 291 | LICENSE.md section #0 |  |  | 0.474 |
| walker |  | 6530 | 51 | python method doc at py3xui/api/api_server.py:97 |  |  | 0.474 |
| walker |  | 6581 | 51 | python method doc at py3xui/async_api/async_api.py:120 |  |  | 0.474 |
| walker |  | 6632 | 51 | python method doc at py3xui/async_api/async_api_server.py:105 |  |  | 0.474 |
| walker |  | 6715 | 83 | python decl names surface in py3xui/utils/env.py |  |  | 0.474 |
| walker |  | 6715 | 0 | python decl at py3xui/utils/env.py:33 |  |  | 0.474 |
| walker |  | 6715 | 0 | python decl at py3xui/utils/env.py:49 |  |  | 0.474 |
| walker |  | 6715 | 0 | python decl at py3xui/utils/env.py:65 |  |  | 0.474 |
| walker |  | 6715 | 0 | python decl at py3xui/utils/env.py:81 |  |  | 0.474 |
| walker |  | 6715 | 0 | python decl at py3xui/utils/env.py:95 |  |  | 0.474 |
| walker |  | 6750 | 35 | python decl at py3xui/utils/env.py:7 |  |  | 0.474 |
| walker |  | 6789 | 39 | python decl body at py3xui/utils/env.py:33 body 43 |  |  | 0.474 |
| walker |  | 6828 | 39 | python decl body at py3xui/utils/env.py:49 body 59 |  |  | 0.474 |
| walker |  | 6867 | 39 | python decl body at py3xui/utils/env.py:65 body 75 |  |  | 0.474 |
| walker |  | 6930 | 63 | python decl doc at py3xui/utils/env.py:81 |  |  | 0.474 |
| walker |  | 6995 | 65 | python decl doc at py3xui/utils/env.py:95 |  |  | 0.474 |
| walker |  | 7077 | 82 | python decl doc at py3xui/utils/env.py:33 |  |  | 0.474 |
| walker |  | 7159 | 82 | python decl doc at py3xui/utils/env.py:49 |  |  | 0.474 |
| walker |  | 7241 | 82 | python decl doc at py3xui/utils/env.py:65 |  |  | 0.474 |
| walker |  | 7291 | 50 | python decl body at py3xui/utils/env.py:95 body 102 |  |  | 0.474 |
| walker |  | 7346 | 55 | python method doc at py3xui/api/api.py:115 |  |  | 0.474 |
| ns | 7374 |  | 979 | AsyncBaseApi _request_with_retry — httpx version | 5.12 | 5.3 | 0.443 |
| walker |  | 7392 | 46 | python imports in py3xui/api/api_database.py |  |  | 0.443 |
| ns | 7482 |  | 108 | utils/env public functions | 6.1 |  | 0.451 |
| walker |  | 7558 | 166 | python class body at py3xui/inbound/inbound.py:15 |  |  | 0.451 |
| walker |  | 7608 | 50 | python imports in py3xui/inbound/sniffing.py |  |  | 0.451 |
| ns | 7632 |  | 150 | Api.from_env body — env var precedence + TLS defaults | 6.2 | 2.7 | 0.445 |
| walker |  | 7701 | 93 | python method at py3xui/api/api.py:67 |  |  | 0.461 |
| walker |  | 7794 | 93 | python method at py3xui/async_api/async_api.py:72 |  |  | 0.476 |
| walker |  | 7857 | 63 | python method doc at py3xui/api/api.py:124 |  |  | 0.476 |
| ns | 7866 |  | 234 | Api.login body — propagates session+cookie_name to sub-APIs | 6.3 | 2.7 | 0.468 |
| walker |  | 7920 | 63 | python method doc at py3xui/async_api/async_api.py:129 |  |  | 0.468 |
| walker |  | 7971 | 51 | python imports in py3xui/async_api/async_api_database.py |  |  | 0.468 |
| walker |  | 8023 | 52 | python imports in py3xui/inbound/stream_settings.py |  |  | 0.468 |
| walker |  | 8088 | 65 | python method doc at py3xui/inbound/bases.py:12 |  |  | 0.476 |
| ns | 8157 |  | 291 | Inbound.validate_stream_settings — dict/str/JSON handling | 6.5 | 4.2 | 0.467 |
| walker |  | 8195 | 107 | python decl names surface in py3xui/server/server.py |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:7 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:43 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:54 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:67 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:78 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:89 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:100 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:113 |  |  | 0.478 |
| walker |  | 8195 | 0 | python decl at py3xui/server/server.py:125 |  |  | 0.478 |
| walker |  | 8213 | 18 | python class body at py3xui/server/server.py:43 |  |  | 0.478 |
| walker |  | 8231 | 18 | python class body at py3xui/server/server.py:67 |  |  | 0.478 |
| walker |  | 8249 | 18 | python class body at py3xui/server/server.py:78 |  |  | 0.478 |
| walker |  | 8263 | 14 | python decl doc at py3xui/server/server.py:7 |  |  | 0.478 |
| walker |  | 8283 | 20 | python class body at py3xui/server/server.py:89 |  |  | 0.478 |
| walker |  | 8303 | 20 | python decl doc at py3xui/server/server.py:113 |  |  | 0.479 |
| walker |  | 8330 | 27 | python class body at py3xui/server/server.py:100 |  |  | 0.479 |
| walker |  | 8373 | 43 | python class body at py3xui/server/server.py:54 |  |  | 0.479 |
| walker |  | 8420 | 47 | python decl doc at py3xui/server/server.py:78 |  |  | 0.479 |
| walker |  | 8469 | 49 | python decl doc at py3xui/server/server.py:67 |  |  | 0.479 |
| walker |  | 8521 | 52 | python decl doc at py3xui/server/server.py:43 |  |  | 0.479 |
| ns | 8543 |  | 386 | Inbound.to_json — XUI API submit format | 6.6 | 4.2 | 0.467 |
| walker |  | 8575 | 54 | python decl doc at py3xui/server/server.py:89 |  |  | 0.467 |
| walker |  | 8636 | 61 | python decl doc at py3xui/server/server.py:100 |  |  | 0.467 |
| walker |  | 8716 | 80 | python class body at py3xui/server/server.py:113 |  |  | 0.476 |
| walker |  | 8787 | 71 | python decl doc at py3xui/server/server.py:54 |  |  | 0.476 |
| ns | 8801 |  | 258 | ClientFields constants roster — JSON-key alias mapping | 6.7 | 4.1 | 0.467 |
| walker |  | 8846 | 59 | README.md section #20 |  |  | 0.467 |
| walker |  | 9061 | 215 | python method sigs in py3xui/api/api_client.py |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:52 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:89 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:121 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:157 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:189 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:217 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:248 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:280 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:312 |  |  | 0.493 |
| walker |  | 9061 | 0 | python method at py3xui/api/api_client.py:342 |  |  | 0.493 |
| ns | 9121 |  | 320 | SettingsFields, SniffingFields, StreamSettingsFields rosters | 6.9 | 4.4 | 0.504 |
| walker |  | 9153 | 92 | py3xui/server/README.md section #2 |  |  | 0.504 |
| ns | 9180 |  | 59 | Test file roster + tests/responses fixtures | 6.10 |  | 0.503 |
| walker |  | 9214 | 61 | README.md section #18 |  |  | 0.503 |
| walker |  | 9439 | 225 | python method sigs in py3xui/async_api/async_api_client.py |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:52 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:89 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:121 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:158 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:190 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:219 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:250 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:282 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:315 |  |  | 0.515 |
| walker |  | 9439 | 0 | python method at py3xui/async_api/async_api_client.py:345 |  |  | 0.515 |
| walker |  | 9505 | 66 | python decl body at py3xui/utils/env.py:81 body 88 |  |  | 0.515 |
| walker |  | 9563 | 58 | python imports in py3xui/api/api_server.py |  |  | 0.515 |
| ns | 9571 |  | 391 | Sync API endpoint URLs — catalog grouped by sub-API | 6.11 | 3.8 | 0.508 |
| walker |  | 9760 | 197 | python class body at py3xui/client/client.py:7 |  |  | 0.527 |
| walker |  | 9768 | 8 | python method body at py3xui/inbound/bases.py:12 body 30 |  |  | 0.528 |
| walker |  | 9831 | 63 | python imports in py3xui/inbound/settings.py |  |  | 0.528 |
| ns | 9905 |  | 334 | README quickstart — env vars + sync/async construction | 6.12 |  | 0.517 |
| walker |  | 9925 | 94 | README.md section #3 |  |  | 0.520 |

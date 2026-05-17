Score(3000)=0.553 I=0.829 C=0.370 ns_rows≤3K=31/58 (reached=16 partial=0 missing=15)

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
| walker |  | 357 | 32 | listing of 'py3xui/inbound' |  |  | 0.744 |
| ns | 378 |  | 80 | Sync sub-API exports | 1.6 |  | 0.688 |
| walker |  | 397 | 40 | listing of 'py3xui/api' |  |  | 0.707 |
| walker |  | 443 | 46 | listing of 'py3xui/async_api' |  |  | 0.732 |
| ns | 469 |  | 91 | Async sub-API exports | 1.7 |  | 0.683 |
| walker |  | 523 | 80 | python imports in py3xui/api/__init__.py |  |  | 0.755 |
| walker |  | 605 | 82 | python imports in py3xui/inbound/__init__.py |  |  | 0.755 |
| ns | 629 |  | 160 | Inner-package directory listings | 1.8 |  | 0.774 |
| ns | 678 |  | 49 | Api class signature + lede | 2.1 |  | 0.756 |
| walker |  | 696 | 91 | python imports in py3xui/async_api/__init__.py |  |  | 0.797 |
| walker |  | 723 | 27 | listing of 'dev' |  |  | 0.797 |
| ns | 728 |  | 50 | AsyncApi class signature + lede | 2.2 |  | 0.780 |
| walker |  | 780 | 57 | [dependencies] in pyproject.toml |  |  | 0.823 |
| ns | 832 |  | 104 | Api constructor signature | 2.3 | 2.1 | 0.775 |
| ns | 936 |  | 104 | AsyncApi constructor signature | 2.4 | 2.2 | 0.734 |
| walker |  | 962 | 182 | README headline in README.md |  |  | 0.734 |
| walker |  | 1012 | 50 | README headline in py3xui/utils/README.md |  |  | 0.734 |
| walker |  | 1064 | 52 | README headline in py3xui/inbound/README.md |  |  | 0.734 |
| walker |  | 1118 | 54 | README headline in py3xui/api/README.md |  |  | 0.734 |
| ns | 1142 |  | 206 | Api sub-client wiring (client/inbound/database/server attrs) | 2.5 | 2.3 | 0.673 |
| walker |  | 1173 | 55 | README headline in py3xui/server/README.md |  |  | 0.673 |
| walker |  | 1229 | 56 | README headline in py3xui/client/README.md |  |  | 0.673 |
| walker |  | 1250 | 21 | headings outline in py3xui/client/README.md |  |  | 0.673 |
| walker |  | 1259 | 9 | python decl names surface in py3xui/api/api.py |  |  | 0.674 |
| walker |  | 1326 | 67 | README headline in py3xui/async_api/README.md |  |  | 0.674 |
| walker |  | 1336 | 10 | python decl names surface in py3xui/async_api/async_api.py |  |  | 0.674 |
| walker |  | 1347 | 11 | python decl names surface in py3xui/async_api/async_api_base.py |  |  | 0.674 |
| ns | 1356 |  | 214 | AsyncApi sub-client wiring | 2.6 | 2.4 | 0.623 |
| walker |  | 1367 | 20 | python decl at py3xui/async_api/async_api_base.py:16 |  |  | 0.623 |
| walker |  | 1379 | 12 | python decl names surface in py3xui/api/api_client.py |  |  | 0.623 |
| walker |  | 1398 | 19 | python decl at py3xui/api/api_client.py:13 |  |  | 0.623 |
| walker |  | 1410 | 12 | python decl names surface in py3xui/api/api_database.py |  |  | 0.623 |
| walker |  | 1429 | 19 | python decl at py3xui/api/api_database.py:7 |  |  | 0.623 |
| walker |  | 1443 | 14 | python method sigs in py3xui/api/api_database.py |  |  | 0.624 |
| walker |  | 1443 | 0 | python method at py3xui/api/api_database.py:32 |  |  | 0.624 |
| walker |  | 1455 | 12 | python decl names surface in py3xui/api/api_server.py |  |  | 0.624 |
| walker |  | 1474 | 19 | python decl at py3xui/api/api_server.py:7 |  |  | 0.624 |
| ns | 1526 |  | 170 | Facade public methods (Api.from_env, Api.login) | 2.7 | 2.1 | 0.589 |
| walker |  | 1539 | 65 | [package] in pyproject.toml |  |  | 0.608 |
| walker |  | 1552 | 13 | python decl names surface in py3xui/api/api_inbound.py |  |  | 0.608 |
| walker |  | 1572 | 20 | python decl at py3xui/api/api_inbound.py:9 |  |  | 0.608 |
| walker |  | 1585 | 13 | python decl names surface in py3xui/inbound/bases.py |  |  | 0.608 |
| walker |  | 1585 | 0 | python decl at py3xui/inbound/bases.py:9 |  |  | 0.608 |
| walker |  | 1599 | 14 | python method sigs in py3xui/inbound/bases.py |  |  | 0.608 |
| walker |  | 1624 | 25 | listing of 'tests' |  |  | 0.609 |
| ns | 1698 |  | 172 | AsyncApi public methods | 2.8 | 2.2 | 0.577 |
| ns | 1789 |  | 91 | ClientApi method roster (sync, names only) | 3.1 |  | 0.558 |
| walker |  | 1791 | 167 | headings outline in README.md |  |  | 0.558 |
| ns | 1851 |  | 62 | InboundApi method roster (sync, names only) | 3.2 |  | 0.545 |
| ns | 1880 |  | 29 | ServerApi method roster (sync, names only) | 3.3 |  | 0.540 |
| ns | 1888 |  | 8 | DatabaseApi method roster (sync, names only) | 3.4 |  | 0.541 |
| walker |  | 1945 | 154 | README.md section #0 |  |  | 0.541 |
| walker |  | 1962 | 17 | python decl doc at py3xui/inbound/bases.py:9 |  |  | 0.541 |
| walker |  | 1977 | 15 | python decl names surface in py3xui/async_api/async_api_client.py |  |  | 0.541 |
| ns | 1989 |  | 101 | AsyncClientApi method roster (async, names only) | 3.5 |  | 0.525 |
| walker |  | 1997 | 20 | python decl at py3xui/async_api/async_api_client.py:12 |  |  | 0.525 |
| walker |  | 2012 | 15 | python decl names surface in py3xui/async_api/async_api_database.py |  |  | 0.525 |
| walker |  | 2031 | 19 | python decl at py3xui/async_api/async_api_database.py:7 |  |  | 0.525 |
| walker |  | 2046 | 15 | python method sigs in py3xui/async_api/async_api_database.py |  |  | 0.525 |
| walker |  | 2046 | 0 | python method at py3xui/async_api/async_api_database.py:32 |  |  | 0.525 |
| ns | 2058 |  | 69 | AsyncInboundApi method roster (async, names only) | 3.6 |  | 0.514 |
| walker |  | 2061 | 15 | python decl names surface in py3xui/async_api/async_api_inbound.py |  |  | 0.514 |
| walker |  | 2081 | 20 | python decl at py3xui/async_api/async_api_inbound.py:11 |  |  | 0.514 |
| walker |  | 2096 | 15 | python decl names surface in py3xui/async_api/async_api_server.py |  |  | 0.514 |
| ns | 2099 |  | 41 | AsyncServerApi + AsyncDatabaseApi method roster | 3.7 |  | 0.509 |
| walker |  | 2125 | 29 | python decl at py3xui/async_api/async_api_server.py:8 |  |  | 0.509 |
| walker |  | 2139 | 14 | python class body at py3xui/async_api/async_api_server.py:8 |  |  | 0.509 |
| walker |  | 2182 | 43 | README.md section #22 |  |  | 0.509 |
| ns | 2203 |  | 104 | ClientApi full signatures (sync) | 3.8 | 3.1 | 0.495 |
| walker |  | 2222 | 40 | python decl at py3xui/api/api.py:13 |  |  | 0.507 |
| walker |  | 2262 | 40 | python decl at py3xui/async_api/async_api.py:18 |  |  | 0.518 |
| ns | 2264 |  | 61 | InboundApi full signatures (sync) | 3.9 | 3.2 | 0.508 |
| walker |  | 2282 | 20 | python decl names surface in py3xui/api/api_base.py |  |  | 0.508 |
| walker |  | 2282 | 0 | python decl at py3xui/api/api_base.py:15 |  |  | 0.508 |
| ns | 2283 |  | 19 | ServerApi full signatures (sync) | 3.10 | 3.3 | 0.504 |
| ns | 2287 |  | 4 | DatabaseApi full signature (sync) | 3.11 | 3.4 | 0.506 |
| walker |  | 2301 | 19 | python decl at py3xui/api/api_base.py:28 |  |  | 0.506 |
| walker |  | 2317 | 16 | python decl doc at py3xui/api/api_base.py:15 |  |  | 0.506 |
| walker |  | 2338 | 21 | python decl names surface in py3xui/client/client.py |  |  | 0.506 |
| walker |  | 2338 | 0 | python decl at py3xui/client/client.py:7 |  |  | 0.506 |
| walker |  | 2352 | 14 | python decl at py3xui/client/client.py:36 |  |  | 0.506 |
| walker |  | 2368 | 16 | python decl doc at py3xui/client/client.py:7 |  |  | 0.506 |
| walker |  | 2390 | 22 | python decl names surface in py3xui/inbound/settings.py |  |  | 0.507 |
| walker |  | 2390 | 0 | python decl at py3xui/inbound/settings.py:9 |  |  | 0.507 |
| walker |  | 2390 | 0 | python decl at py3xui/inbound/settings.py:17 |  |  | 0.507 |
| walker |  | 2406 | 16 | python decl doc at py3xui/inbound/settings.py:9 |  |  | 0.507 |
| ns | 2429 |  | 142 | Client model — fields list (field names only) | 4.1 |  | 0.484 |
| walker |  | 2441 | 35 | python class body at py3xui/inbound/settings.py:17 |  |  | 0.484 |
| walker |  | 2481 | 40 | python imports in demo.py |  |  | 0.484 |
| walker |  | 2504 | 23 | python decl names surface in py3xui/inbound/inbound.py |  |  | 0.484 |
| walker |  | 2504 | 0 | python decl at py3xui/inbound/inbound.py:15 |  |  | 0.484 |
| walker |  | 2519 | 15 | python decl at py3xui/inbound/inbound.py:38 |  |  | 0.484 |
| walker |  | 2535 | 16 | python decl doc at py3xui/inbound/inbound.py:15 |  |  | 0.484 |
| walker |  | 2567 | 32 | python method sigs in py3xui/inbound/inbound.py |  |  | 0.484 |
| walker |  | 2567 | 0 | python method at py3xui/inbound/inbound.py:114 |  |  | 0.484 |
| ns | 2576 |  | 147 | Inbound model — fields list | 4.2 |  | 0.467 |
| walker |  | 2604 | 37 | python class body at py3xui/inbound/settings.py:9 |  |  | 0.467 |
| walker |  | 2628 | 24 | python decl names surface in py3xui/inbound/stream_settings.py |  |  | 0.467 |
| walker |  | 2628 | 0 | python decl at py3xui/inbound/stream_settings.py:9 |  |  | 0.467 |
| walker |  | 2628 | 0 | python decl at py3xui/inbound/stream_settings.py:25 |  |  | 0.467 |
| walker |  | 2644 | 16 | python decl doc at py3xui/inbound/stream_settings.py:9 |  |  | 0.467 |
| ns | 2657 |  | 81 | Server, MemoryInfo, XRayInfo, NetworkIO, NetworkTraffic, PublicIP, AppStats, RealityKeyPair class headers | 4.3 |  | 0.459 |
| walker |  | 2670 | 26 | python decl names surface in py3xui/inbound/sniffing.py |  |  | 0.460 |
| walker |  | 2670 | 0 | python decl at py3xui/inbound/sniffing.py:9 |  |  | 0.460 |
| walker |  | 2670 | 0 | python decl at py3xui/inbound/sniffing.py:20 |  |  | 0.460 |
| walker |  | 2686 | 16 | python decl doc at py3xui/inbound/sniffing.py:9 |  |  | 0.460 |
| ns | 2701 |  | 44 | Settings, Sniffing, StreamSettings, JsonStringModel class headers | 4.4 |  | 0.466 |
| walker |  | 2708 | 22 | README.md section #2 |  |  | 0.466 |
| walker |  | 2762 | 54 | python method sigs in py3xui/api/api_server.py |  |  | 0.483 |
| walker |  | 2762 | 0 | python method at py3xui/api/api_server.py:35 |  |  | 0.483 |
| walker |  | 2762 | 0 | python method at py3xui/api/api_server.py:67 |  |  | 0.483 |
| walker |  | 2762 | 0 | python method at py3xui/api/api_server.py:97 |  |  | 0.483 |
| walker |  | 2862 | 100 | headings outline in py3xui/server/README.md |  |  | 0.483 |
| walker |  | 2919 | 57 | python method sigs in py3xui/async_api/async_api_server.py |  |  | 0.494 |
| walker |  | 2919 | 0 | python method at py3xui/async_api/async_api_server.py:44 |  |  | 0.494 |
| walker |  | 2919 | 0 | python method at py3xui/async_api/async_api_server.py:76 |  |  | 0.494 |
| walker |  | 2919 | 0 | python method at py3xui/async_api/async_api_server.py:105 |  |  | 0.494 |
| walker |  | 2968 | 49 | python class body at py3xui/inbound/sniffing.py:9 |  |  | 0.494 |
| ns | 3001 |  | 300 | StreamSettings full field list (network/security/tcp/kcp/reality/xtls/tls/xhttp) | 4.5 |  | 0.471 |
| walker |  | 3009 | 41 | python method at py3xui/inbound/bases.py:12 |  |  | 0.472 |
| walker |  | 3059 | 50 | py3xui/server/README.md section #1 |  |  | 0.472 |
| walker |  | 3103 | 44 | python method at py3xui/inbound/inbound.py:88 |  |  | 0.472 |
| walker |  | 3154 | 51 | py3xui/client/README.md section #1 |  |  | 0.472 |
| walker |  | 3297 | 143 | headings outline in py3xui/api/README.md |  |  | 0.472 |
| ns | 3309 |  | 308 | Server full field list | 4.6 | 4.3 | 0.458 |
| walker |  | 3448 | 151 | headings outline in py3xui/inbound/README.md |  |  | 0.458 |
| ns | 3455 |  | 146 | Settings + Sniffing full field lists | 4.7 | 4.4 | 0.452 |
| walker |  | 3611 | 163 | headings outline in py3xui/async_api/README.md |  |  | 0.452 |
| ns | 3639 |  | 184 | JsonStringModel preprocessing validator | 4.8 | 4.4 | 0.440 |
| ns | 3751 |  | 112 | RealityKeyPair full body | 4.9 | 4.3 | 0.433 |
| walker |  | 3789 | 178 | python decl names surface in demo.py |  |  | 0.433 |
| ns | 3863 |  | 112 | ApiFields constants (response keys + HTTP methods) | 5.1 |  | 0.426 |
| walker |  | 3865 | 76 | python class body at py3xui/api/api_base.py:15 |  |  | 0.444 |
| ns | 3892 |  | 29 | BaseApi class signature + summary | 5.2 |  | 0.447 |
| ns | 3923 |  | 31 | AsyncBaseApi class signature | 5.3 |  | 0.449 |
| walker |  | 3955 | 90 | python method sigs in py3xui/api/api.py |  |  | 0.455 |
| walker |  | 3955 | 0 | python method at py3xui/api/api.py:189 |  |  | 0.455 |
| walker |  | 3967 | 12 | python method at py3xui/api/api.py:93 |  |  | 0.455 |
| walker |  | 3980 | 13 | python method at py3xui/api/api.py:115 |  |  | 0.455 |
| walker |  | 3994 | 14 | python method at py3xui/api/api.py:102 |  |  | 0.457 |
| walker |  | 4011 | 17 | python method at py3xui/api/api.py:124 |  |  | 0.461 |
| walker |  | 4021 | 10 | python method body at py3xui/api/api.py:93 body 100 |  |  | 0.461 |
| walker |  | 4032 | 11 | python method body at py3xui/api/api.py:115 body 122 |  |  | 0.461 |
| ns | 4050 |  | 127 | BaseApi __init__ field set | 5.4 | 5.2 | 0.454 |
| walker |  | 4123 | 91 | python method sigs in py3xui/async_api/async_api.py |  |  | 0.460 |
| walker |  | 4123 | 0 | python method at py3xui/async_api/async_api.py:194 |  |  | 0.460 |
| walker |  | 4135 | 12 | python method at py3xui/async_api/async_api.py:98 |  |  | 0.460 |
| walker |  | 4148 | 13 | python method at py3xui/async_api/async_api.py:120 |  |  | 0.460 |
| walker |  | 4162 | 14 | python method at py3xui/async_api/async_api.py:107 |  |  | 0.462 |
| ns | 4174 |  | 124 | BaseApi property names roster (host/username/password/use_tls_verify/custom_certificate_path/max_retries/session/cookie_name/cookies) | 5.5 | 5.2 | 0.456 |
| walker |  | 4179 | 17 | python method at py3xui/async_api/async_api.py:129 |  |  | 0.459 |
| walker |  | 4189 | 10 | python method body at py3xui/async_api/async_api.py:98 body 105 |  |  | 0.459 |
| walker |  | 4200 | 11 | python method body at py3xui/async_api/async_api.py:120 body 127 |  |  | 0.459 |
| walker |  | 4243 | 43 | README.md section #7 |  |  | 0.459 |
| ns | 4391 |  | 217 | BaseApi method roster (login, _get_cookie, _check_response, _url, _request_with_retry, _post, _get) | 5.6 | 5.2 | 0.450 |
| walker |  | 4440 | 197 | py3xui/inbound/README.md section #0 |  |  | 0.450 |
| ns | 4507 |  | 116 | COOKIE_NAMES and cookies property | 5.7 | 5.5 | 0.443 |
| walker |  | 4594 | 154 | README.md section #1 |  |  | 0.443 |
| walker |  | 4720 | 126 | python decl names surface #1 in demo.py |  |  | 0.443 |
| walker |  | 4720 | 0 | python decl at demo.py:70 |  |  | 0.443 |
| walker |  | 4815 | 95 | python decl doc at demo.py:70 |  |  | 0.443 |
| walker |  | 4916 | 101 | python class body at py3xui/inbound/sniffing.py:20 |  |  | 0.456 |
| ns | 4968 |  | 461 | BaseApi.login body — sends credentials, captures cookie | 5.8 | 5.6 | 0.431 |
| walker |  | 4982 | 66 | python method at py3xui/api/api.py:139 |  |  | 0.449 |
| walker |  | 5049 | 67 | python method at py3xui/async_api/async_api.py:144 |  |  | 0.467 |
| walker |  | 5082 | 33 | python imports in py3xui/utils/env.py |  |  | 0.467 |
| walker |  | 5234 | 152 | README.md section #12 |  |  | 0.467 |
| walker |  | 5347 | 113 | python class body at py3xui/inbound/stream_settings.py:9 |  |  | 0.468 |
| walker |  | 5434 | 87 | python decl doc at py3xui/inbound/settings.py:17 |  |  | 0.468 |
| walker |  | 5571 | 137 | python method sigs in py3xui/api/api_inbound.py |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:40 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:71 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:111 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:159 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:190 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:223 |  |  | 0.493 |
| walker |  | 5571 | 0 | python method at py3xui/api/api_inbound.py:247 |  |  | 0.493 |
| walker |  | 5622 | 51 | README.md section #16 |  |  | 0.493 |
| walker |  | 5666 | 44 | python method doc at py3xui/api/api.py:93 |  |  | 0.493 |
| walker |  | 5710 | 44 | python method doc at py3xui/api/api.py:102 |  |  | 0.493 |
| walker |  | 5754 | 44 | python method doc at py3xui/async_api/async_api.py:98 |  |  | 0.493 |
| walker |  | 5798 | 44 | python method doc at py3xui/async_api/async_api.py:107 |  |  | 0.456 |
| ns | 5798 |  | 830 | _request_with_retry — TLS verify branch + retry/backoff body (sync) | 5.9 | 5.6 | 0.456 |
| walker |  | 5942 | 144 | python method sigs in py3xui/async_api/async_api_inbound.py |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:42 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:72 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:112 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:158 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:189 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:222 |  |  | 0.467 |
| walker |  | 5942 | 0 | python method at py3xui/async_api/async_api_inbound.py:246 |  |  | 0.467 |
| walker |  | 5979 | 37 | python imports in py3xui/server/server.py |  |  | 0.467 |
| walker |  | 6076 | 97 | python decl doc at py3xui/inbound/sniffing.py:20 |  |  | 0.467 |
| walker |  | 6114 | 38 | python imports in py3xui/client/client.py |  |  | 0.467 |
| walker |  | 6152 | 38 | python imports in py3xui/inbound/bases.py |  |  | 0.467 |
| walker |  | 6201 | 49 | python method doc at py3xui/inbound/inbound.py:114 |  |  | 0.467 |
| ns | 6238 |  | 440 | _post and _get bodies — login gate + delegation to retry | 5.10 | 5.6 | 0.450 |
| ns | 6395 |  | 157 | _check_response — success-field validation | 5.11 | 5.6 | 0.443 |
| walker |  | 6492 | 291 | LICENSE.md section #0 |  |  | 0.443 |
| walker |  | 6543 | 51 | python method doc at py3xui/api/api_server.py:97 |  |  | 0.443 |
| walker |  | 6594 | 51 | python method doc at py3xui/async_api/async_api.py:120 |  |  | 0.443 |
| walker |  | 6645 | 51 | python method doc at py3xui/async_api/async_api_server.py:105 |  |  | 0.443 |
| walker |  | 6728 | 83 | python decl names surface in py3xui/utils/env.py |  |  | 0.443 |
| walker |  | 6728 | 0 | python decl at py3xui/utils/env.py:33 |  |  | 0.443 |
| walker |  | 6728 | 0 | python decl at py3xui/utils/env.py:49 |  |  | 0.443 |
| walker |  | 6728 | 0 | python decl at py3xui/utils/env.py:65 |  |  | 0.443 |
| walker |  | 6728 | 0 | python decl at py3xui/utils/env.py:81 |  |  | 0.443 |
| walker |  | 6728 | 0 | python decl at py3xui/utils/env.py:95 |  |  | 0.443 |
| walker |  | 6794 | 66 | python decl at py3xui/utils/env.py:7 |  |  | 0.443 |
| walker |  | 6833 | 39 | python decl body at py3xui/utils/env.py:33 body 43 |  |  | 0.443 |
| walker |  | 6872 | 39 | python decl body at py3xui/utils/env.py:49 body 59 |  |  | 0.443 |
| walker |  | 6911 | 39 | python decl body at py3xui/utils/env.py:65 body 75 |  |  | 0.443 |
| walker |  | 6974 | 63 | python decl doc at py3xui/utils/env.py:81 |  |  | 0.443 |
| walker |  | 7039 | 65 | python decl doc at py3xui/utils/env.py:95 |  |  | 0.443 |
| walker |  | 7121 | 82 | python decl doc at py3xui/utils/env.py:33 |  |  | 0.443 |
| walker |  | 7203 | 82 | python decl doc at py3xui/utils/env.py:49 |  |  | 0.443 |
| walker |  | 7285 | 82 | python decl doc at py3xui/utils/env.py:65 |  |  | 0.443 |
| walker |  | 7335 | 50 | python decl body at py3xui/utils/env.py:95 body 102 |  |  | 0.443 |
| ns | 7374 |  | 979 | AsyncBaseApi _request_with_retry — httpx version | 5.12 | 5.3 | 0.414 |
| walker |  | 7390 | 55 | python method doc at py3xui/api/api.py:115 |  |  | 0.414 |
| walker |  | 7436 | 46 | python imports in py3xui/api/api_database.py |  |  | 0.414 |
| ns | 7482 |  | 108 | utils/env public functions | 6.1 |  | 0.421 |
| walker |  | 7602 | 166 | python class body at py3xui/inbound/inbound.py:15 |  |  | 0.421 |
| ns | 7632 |  | 150 | Api.from_env body — env var precedence + TLS defaults | 6.2 | 2.7 | 0.416 |
| walker |  | 7652 | 50 | python imports in py3xui/inbound/sniffing.py |  |  | 0.416 |
| walker |  | 7745 | 93 | python method at py3xui/api/api.py:67 |  |  | 0.431 |
| walker |  | 7838 | 93 | python method at py3xui/async_api/async_api.py:72 |  |  | 0.445 |
| ns | 7866 |  | 234 | Api.login body — propagates session+cookie_name to sub-APIs | 6.3 | 2.7 | 0.437 |
| walker |  | 7901 | 63 | python method doc at py3xui/api/api.py:124 |  |  | 0.437 |
| walker |  | 7964 | 63 | python method doc at py3xui/async_api/async_api.py:129 |  |  | 0.437 |
| walker |  | 8015 | 51 | python imports in py3xui/async_api/async_api_database.py |  |  | 0.437 |
| walker |  | 8067 | 52 | python imports in py3xui/inbound/stream_settings.py |  |  | 0.437 |
| walker |  | 8132 | 65 | python method doc at py3xui/inbound/bases.py:12 |  |  | 0.445 |
| ns | 8157 |  | 291 | Inbound.validate_stream_settings — dict/str/JSON handling | 6.5 | 4.2 | 0.437 |
| walker |  | 8239 | 107 | python decl names surface in py3xui/server/server.py |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:7 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:43 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:54 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:67 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:78 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:89 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:100 |  |  | 0.447 |
| walker |  | 8239 | 0 | python decl at py3xui/server/server.py:113 |  |  | 0.447 |
| walker |  | 8252 | 13 | python decl at py3xui/server/server.py:125 |  |  | 0.447 |
| walker |  | 8270 | 18 | python class body at py3xui/server/server.py:43 |  |  | 0.447 |
| walker |  | 8288 | 18 | python class body at py3xui/server/server.py:67 |  |  | 0.447 |
| walker |  | 8306 | 18 | python class body at py3xui/server/server.py:78 |  |  | 0.447 |
| walker |  | 8320 | 14 | python decl doc at py3xui/server/server.py:7 |  |  | 0.447 |
| walker |  | 8340 | 20 | python class body at py3xui/server/server.py:89 |  |  | 0.447 |
| walker |  | 8360 | 20 | python decl doc at py3xui/server/server.py:113 |  |  | 0.448 |
| walker |  | 8387 | 27 | python class body at py3xui/server/server.py:100 |  |  | 0.448 |
| walker |  | 8430 | 43 | python class body at py3xui/server/server.py:54 |  |  | 0.448 |
| walker |  | 8477 | 47 | python decl doc at py3xui/server/server.py:78 |  |  | 0.448 |
| walker |  | 8526 | 49 | python decl doc at py3xui/server/server.py:67 |  |  | 0.448 |
| ns | 8543 |  | 386 | Inbound.to_json — XUI API submit format | 6.6 | 4.2 | 0.436 |
| walker |  | 8578 | 52 | python decl doc at py3xui/server/server.py:43 |  |  | 0.436 |
| walker |  | 8632 | 54 | python decl doc at py3xui/server/server.py:89 |  |  | 0.436 |
| walker |  | 8693 | 61 | python decl doc at py3xui/server/server.py:100 |  |  | 0.436 |
| walker |  | 8773 | 80 | python class body at py3xui/server/server.py:113 |  |  | 0.445 |
| ns | 8801 |  | 258 | ClientFields constants roster — JSON-key alias mapping | 6.7 | 4.1 | 0.436 |
| walker |  | 8844 | 71 | python decl doc at py3xui/server/server.py:54 |  |  | 0.436 |
| walker |  | 8903 | 59 | README.md section #20 |  |  | 0.436 |
| walker |  | 9118 | 215 | python method sigs in py3xui/api/api_client.py |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:52 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:89 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:121 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:157 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:189 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:217 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:248 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:280 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:312 |  |  | 0.460 |
| walker |  | 9118 | 0 | python method at py3xui/api/api_client.py:342 |  |  | 0.460 |
| ns | 9121 |  | 320 | SettingsFields, SniffingFields, StreamSettingsFields rosters | 6.9 | 4.4 | 0.470 |
| ns | 9180 |  | 59 | Test file roster + tests/responses fixtures | 6.10 |  | 0.469 |
| walker |  | 9210 | 92 | py3xui/server/README.md section #2 |  |  | 0.469 |
| walker |  | 9271 | 61 | README.md section #18 |  |  | 0.469 |
| walker |  | 9496 | 225 | python method sigs in py3xui/async_api/async_api_client.py |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:52 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:89 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:121 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:158 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:190 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:219 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:250 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:282 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:315 |  |  | 0.481 |
| walker |  | 9496 | 0 | python method at py3xui/async_api/async_api_client.py:345 |  |  | 0.481 |
| walker |  | 9562 | 66 | python decl body at py3xui/utils/env.py:81 body 88 |  |  | 0.481 |
| ns | 9571 |  | 391 | Sync API endpoint URLs — catalog grouped by sub-API | 6.11 | 3.8 | 0.474 |
| walker |  | 9620 | 58 | python imports in py3xui/api/api_server.py |  |  | 0.474 |
| walker |  | 9817 | 197 | python class body at py3xui/client/client.py:7 |  |  | 0.491 |
| walker |  | 9825 | 8 | python method body at py3xui/inbound/bases.py:12 body 30 |  |  | 0.492 |
| walker |  | 9888 | 63 | python imports in py3xui/inbound/settings.py |  |  | 0.492 |
| ns | 9905 |  | 334 | README quickstart — env vars + sync/async construction | 6.12 |  | 0.482 |
| walker |  | 9982 | 94 | README.md section #3 |  |  | 0.485 |

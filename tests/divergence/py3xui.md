Score(3000)=0.553 I=0.827 C=0.370 ns_rows≤3K=31/58 (reached=16 partial=0 missing=15)

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
| walker |  | 830 | 50 | README headline in py3xui/utils/README.md |  |  | 0.823 |
| ns | 832 |  | 104 | Api constructor signature | 2.3 | 2.1 | 0.775 |
| walker |  | 882 | 52 | README headline in py3xui/inbound/README.md |  |  | 0.775 |
| walker |  | 936 | 54 | README headline in py3xui/api/README.md |  |  | 0.734 |
| ns | 936 |  | 104 | AsyncApi constructor signature | 2.4 | 2.2 | 0.734 |
| walker |  | 991 | 55 | README headline in py3xui/server/README.md |  |  | 0.734 |
| walker |  | 1047 | 56 | README headline in py3xui/client/README.md |  |  | 0.734 |
| walker |  | 1068 | 21 | headings outline in py3xui/client/README.md |  |  | 0.734 |
| ns | 1142 |  | 206 | Api sub-client wiring (client/inbound/database/server attrs) | 2.5 | 2.3 | 0.673 |
| walker |  | 1312 | 244 | README headline in README.md |  |  | 0.761 |
| walker |  | 1321 | 9 | python decl names surface in py3xui/api/api.py |  |  | 0.761 |
| ns | 1356 |  | 214 | AsyncApi sub-client wiring | 2.6 | 2.4 | 0.703 |
| walker |  | 1388 | 67 | README headline in py3xui/async_api/README.md |  |  | 0.703 |
| walker |  | 1398 | 10 | python decl names surface in py3xui/async_api/async_api.py |  |  | 0.704 |
| walker |  | 1409 | 11 | python decl names surface in py3xui/async_api/async_api_base.py |  |  | 0.704 |
| walker |  | 1429 | 20 | python decl at py3xui/async_api/async_api_base.py:16 |  |  | 0.704 |
| walker |  | 1441 | 12 | python decl names surface in py3xui/api/api_client.py |  |  | 0.704 |
| walker |  | 1460 | 19 | python decl at py3xui/api/api_client.py:13 |  |  | 0.704 |
| walker |  | 1472 | 12 | python decl names surface in py3xui/api/api_database.py |  |  | 0.704 |
| walker |  | 1491 | 19 | python decl at py3xui/api/api_database.py:7 |  |  | 0.704 |
| walker |  | 1505 | 14 | python method sigs in py3xui/api/api_database.py |  |  | 0.705 |
| walker |  | 1505 | 0 | python method at py3xui/api/api_database.py:32 |  |  | 0.705 |
| walker |  | 1517 | 12 | python decl names surface in py3xui/api/api_server.py |  |  | 0.705 |
| ns | 1526 |  | 170 | Facade public methods (Api.from_env, Api.login) | 2.7 | 2.1 | 0.665 |
| walker |  | 1536 | 19 | python decl at py3xui/api/api_server.py:7 |  |  | 0.665 |
| walker |  | 1601 | 65 | [package] in pyproject.toml |  |  | 0.685 |
| walker |  | 1614 | 13 | python decl names surface in py3xui/api/api_inbound.py |  |  | 0.685 |
| walker |  | 1634 | 20 | python decl at py3xui/api/api_inbound.py:9 |  |  | 0.685 |
| walker |  | 1647 | 13 | python decl names surface in py3xui/inbound/bases.py |  |  | 0.685 |
| walker |  | 1647 | 0 | python decl at py3xui/inbound/bases.py:9 |  |  | 0.685 |
| walker |  | 1661 | 14 | python method sigs in py3xui/inbound/bases.py |  |  | 0.685 |
| walker |  | 1686 | 25 | listing of 'tests' |  |  | 0.685 |
| ns | 1698 |  | 172 | AsyncApi public methods | 2.8 | 2.2 | 0.650 |
| ns | 1789 |  | 91 | ClientApi method roster (sync, names only) | 3.1 |  | 0.628 |
| ns | 1851 |  | 62 | InboundApi method roster (sync, names only) | 3.2 |  | 0.614 |
| walker |  | 1853 | 167 | headings outline in README.md |  |  | 0.614 |
| ns | 1880 |  | 29 | ServerApi method roster (sync, names only) | 3.3 |  | 0.608 |
| ns | 1888 |  | 8 | DatabaseApi method roster (sync, names only) | 3.4 |  | 0.610 |
| ns | 1989 |  | 101 | AsyncClientApi method roster (async, names only) | 3.5 |  | 0.591 |
| walker |  | 2007 | 154 | README.md section #0 |  |  | 0.591 |
| walker |  | 2024 | 17 | python decl doc at py3xui/inbound/bases.py:9 |  |  | 0.591 |
| walker |  | 2039 | 15 | python decl names surface in py3xui/async_api/async_api_client.py |  |  | 0.591 |
| ns | 2058 |  | 69 | AsyncInboundApi method roster (async, names only) | 3.6 |  | 0.579 |
| walker |  | 2059 | 20 | python decl at py3xui/async_api/async_api_client.py:12 |  |  | 0.579 |
| walker |  | 2074 | 15 | python decl names surface in py3xui/async_api/async_api_database.py |  |  | 0.579 |
| walker |  | 2093 | 19 | python decl at py3xui/async_api/async_api_database.py:7 |  |  | 0.579 |
| ns | 2099 |  | 41 | AsyncServerApi + AsyncDatabaseApi method roster | 3.7 |  | 0.572 |
| walker |  | 2108 | 15 | python method sigs in py3xui/async_api/async_api_database.py |  |  | 0.573 |
| walker |  | 2108 | 0 | python method at py3xui/async_api/async_api_database.py:32 |  |  | 0.573 |
| walker |  | 2123 | 15 | python decl names surface in py3xui/async_api/async_api_inbound.py |  |  | 0.573 |
| walker |  | 2143 | 20 | python decl at py3xui/async_api/async_api_inbound.py:11 |  |  | 0.573 |
| walker |  | 2158 | 15 | python decl names surface in py3xui/async_api/async_api_server.py |  |  | 0.573 |
| walker |  | 2187 | 29 | python decl at py3xui/async_api/async_api_server.py:8 |  |  | 0.573 |
| walker |  | 2201 | 14 | python class body at py3xui/async_api/async_api_server.py:8 |  |  | 0.573 |
| ns | 2203 |  | 104 | ClientApi full signatures (sync) | 3.8 | 3.1 | 0.558 |
| walker |  | 2244 | 43 | README.md section #22 |  |  | 0.558 |
| ns | 2264 |  | 61 | InboundApi full signatures (sync) | 3.9 | 3.2 | 0.547 |
| ns | 2283 |  | 19 | ServerApi full signatures (sync) | 3.10 | 3.3 | 0.543 |
| walker |  | 2284 | 40 | python decl at py3xui/api/api.py:13 |  |  | 0.555 |
| ns | 2287 |  | 4 | DatabaseApi full signature (sync) | 3.11 | 3.4 | 0.557 |
| walker |  | 2324 | 40 | python decl at py3xui/async_api/async_api.py:18 |  |  | 0.568 |
| walker |  | 2344 | 20 | python decl names surface in py3xui/api/api_base.py |  |  | 0.568 |
| walker |  | 2344 | 0 | python decl at py3xui/api/api_base.py:15 |  |  | 0.568 |
| walker |  | 2363 | 19 | python decl at py3xui/api/api_base.py:28 |  |  | 0.569 |
| walker |  | 2379 | 16 | python decl doc at py3xui/api/api_base.py:15 |  |  | 0.569 |
| walker |  | 2400 | 21 | python decl names surface in py3xui/client/client.py |  |  | 0.569 |
| walker |  | 2400 | 0 | python decl at py3xui/client/client.py:7 |  |  | 0.569 |
| walker |  | 2414 | 14 | python decl at py3xui/client/client.py:36 |  |  | 0.569 |
| ns | 2429 |  | 142 | Client model — fields list (field names only) | 4.1 |  | 0.543 |
| walker |  | 2430 | 16 | python decl doc at py3xui/client/client.py:7 |  |  | 0.543 |
| walker |  | 2452 | 22 | python decl names surface in py3xui/inbound/settings.py |  |  | 0.543 |
| walker |  | 2452 | 0 | python decl at py3xui/inbound/settings.py:9 |  |  | 0.543 |
| walker |  | 2452 | 0 | python decl at py3xui/inbound/settings.py:17 |  |  | 0.543 |
| walker |  | 2468 | 16 | python decl doc at py3xui/inbound/settings.py:9 |  |  | 0.543 |
| walker |  | 2503 | 35 | python class body at py3xui/inbound/settings.py:17 |  |  | 0.543 |
| walker |  | 2543 | 40 | python imports in demo.py |  |  | 0.543 |
| walker |  | 2566 | 23 | python decl names surface in py3xui/inbound/inbound.py |  |  | 0.543 |
| walker |  | 2566 | 0 | python decl at py3xui/inbound/inbound.py:15 |  |  | 0.543 |
| ns | 2576 |  | 147 | Inbound model — fields list | 4.2 |  | 0.523 |
| walker |  | 2581 | 15 | python decl at py3xui/inbound/inbound.py:38 |  |  | 0.523 |
| walker |  | 2597 | 16 | python decl doc at py3xui/inbound/inbound.py:15 |  |  | 0.523 |
| walker |  | 2629 | 32 | python method sigs in py3xui/inbound/inbound.py |  |  | 0.524 |
| walker |  | 2629 | 0 | python method at py3xui/inbound/inbound.py:114 |  |  | 0.524 |
| ns | 2657 |  | 81 | Server, MemoryInfo, XRayInfo, NetworkIO, NetworkTraffic, PublicIP, AppStats, RealityKeyPair class headers | 4.3 |  | 0.515 |
| walker |  | 2666 | 37 | python class body at py3xui/inbound/settings.py:9 |  |  | 0.515 |
| walker |  | 2690 | 24 | python decl names surface in py3xui/inbound/stream_settings.py |  |  | 0.516 |
| walker |  | 2690 | 0 | python decl at py3xui/inbound/stream_settings.py:9 |  |  | 0.516 |
| walker |  | 2690 | 0 | python decl at py3xui/inbound/stream_settings.py:25 |  |  | 0.516 |
| ns | 2701 |  | 44 | Settings, Sniffing, StreamSettings, JsonStringModel class headers | 4.4 |  | 0.518 |
| walker |  | 2706 | 16 | python decl doc at py3xui/inbound/stream_settings.py:9 |  |  | 0.518 |
| walker |  | 2732 | 26 | python decl names surface in py3xui/inbound/sniffing.py |  |  | 0.523 |
| walker |  | 2732 | 0 | python decl at py3xui/inbound/sniffing.py:9 |  |  | 0.523 |
| walker |  | 2732 | 0 | python decl at py3xui/inbound/sniffing.py:20 |  |  | 0.523 |
| walker |  | 2748 | 16 | python decl doc at py3xui/inbound/sniffing.py:9 |  |  | 0.523 |
| walker |  | 2770 | 22 | README.md section #2 |  |  | 0.523 |
| walker |  | 2824 | 54 | python method sigs in py3xui/api/api_server.py |  |  | 0.542 |
| walker |  | 2824 | 0 | python method at py3xui/api/api_server.py:35 |  |  | 0.542 |
| walker |  | 2824 | 0 | python method at py3xui/api/api_server.py:67 |  |  | 0.542 |
| walker |  | 2824 | 0 | python method at py3xui/api/api_server.py:97 |  |  | 0.542 |
| walker |  | 2924 | 100 | headings outline in py3xui/server/README.md |  |  | 0.542 |
| walker |  | 2981 | 57 | python method sigs in py3xui/async_api/async_api_server.py |  |  | 0.553 |
| walker |  | 2981 | 0 | python method at py3xui/async_api/async_api_server.py:44 |  |  | 0.553 |
| walker |  | 2981 | 0 | python method at py3xui/async_api/async_api_server.py:76 |  |  | 0.553 |
| walker |  | 2981 | 0 | python method at py3xui/async_api/async_api_server.py:105 |  |  | 0.553 |
| ns | 3001 |  | 300 | StreamSettings full field list (network/security/tcp/kcp/reality/xtls/tls/xhttp) | 4.5 |  | 0.528 |
| walker |  | 3030 | 49 | python class body at py3xui/inbound/sniffing.py:9 |  |  | 0.528 |
| walker |  | 3071 | 41 | python method at py3xui/inbound/bases.py:12 |  |  | 0.528 |
| walker |  | 3121 | 50 | py3xui/server/README.md section #1 |  |  | 0.528 |
| walker |  | 3165 | 44 | python method at py3xui/inbound/inbound.py:88 |  |  | 0.528 |
| walker |  | 3216 | 51 | py3xui/client/README.md section #1 |  |  | 0.528 |
| ns | 3309 |  | 308 | Server full field list | 4.6 | 4.3 | 0.513 |
| walker |  | 3359 | 143 | headings outline in py3xui/api/README.md |  |  | 0.513 |
| ns | 3455 |  | 146 | Settings + Sniffing full field lists | 4.7 | 4.4 | 0.506 |
| walker |  | 3510 | 151 | headings outline in py3xui/inbound/README.md |  |  | 0.506 |
| ns | 3639 |  | 184 | JsonStringModel preprocessing validator | 4.8 | 4.4 | 0.493 |
| walker |  | 3673 | 163 | headings outline in py3xui/async_api/README.md |  |  | 0.493 |
| ns | 3751 |  | 112 | RealityKeyPair full body | 4.9 | 4.3 | 0.484 |
| walker |  | 3851 | 178 | python decl names surface in demo.py |  |  | 0.484 |
| ns | 3863 |  | 112 | ApiFields constants (response keys + HTTP methods) | 5.1 |  | 0.478 |
| ns | 3892 |  | 29 | BaseApi class signature + summary | 5.2 |  | 0.481 |
| ns | 3923 |  | 31 | AsyncBaseApi class signature | 5.3 |  | 0.484 |
| walker |  | 3927 | 76 | python class body at py3xui/api/api_base.py:15 |  |  | 0.503 |
| walker |  | 4017 | 90 | python method sigs in py3xui/api/api.py |  |  | 0.509 |
| walker |  | 4017 | 0 | python method at py3xui/api/api.py:189 |  |  | 0.509 |
| walker |  | 4029 | 12 | python method at py3xui/api/api.py:93 |  |  | 0.509 |
| walker |  | 4042 | 13 | python method at py3xui/api/api.py:115 |  |  | 0.509 |
| ns | 4050 |  | 127 | BaseApi __init__ field set | 5.4 | 5.2 | 0.502 |
| walker |  | 4056 | 14 | python method at py3xui/api/api.py:102 |  |  | 0.504 |
| walker |  | 4073 | 17 | python method at py3xui/api/api.py:124 |  |  | 0.508 |
| walker |  | 4083 | 10 | python method body at py3xui/api/api.py:93 body 100 |  |  | 0.508 |
| walker |  | 4094 | 11 | python method body at py3xui/api/api.py:115 body 122 |  |  | 0.508 |
| ns | 4174 |  | 124 | BaseApi property names roster (host/username/password/use_tls_verify/custom_certificate_path/max_retries/session/cookie_name/cookies) | 5.5 | 5.2 | 0.501 |
| walker |  | 4185 | 91 | python method sigs in py3xui/async_api/async_api.py |  |  | 0.507 |
| walker |  | 4185 | 0 | python method at py3xui/async_api/async_api.py:194 |  |  | 0.507 |
| walker |  | 4197 | 12 | python method at py3xui/async_api/async_api.py:98 |  |  | 0.507 |
| walker |  | 4210 | 13 | python method at py3xui/async_api/async_api.py:120 |  |  | 0.507 |
| walker |  | 4224 | 14 | python method at py3xui/async_api/async_api.py:107 |  |  | 0.509 |
| walker |  | 4241 | 17 | python method at py3xui/async_api/async_api.py:129 |  |  | 0.512 |
| walker |  | 4251 | 10 | python method body at py3xui/async_api/async_api.py:98 body 105 |  |  | 0.512 |
| walker |  | 4262 | 11 | python method body at py3xui/async_api/async_api.py:120 body 127 |  |  | 0.512 |
| walker |  | 4305 | 43 | README.md section #7 |  |  | 0.512 |
| ns | 4391 |  | 217 | BaseApi method roster (login, _get_cookie, _check_response, _url, _request_with_retry, _post, _get) | 5.6 | 5.2 | 0.502 |
| walker |  | 4502 | 197 | py3xui/inbound/README.md section #0 |  |  | 0.502 |
| ns | 4507 |  | 116 | COOKIE_NAMES and cookies property | 5.7 | 5.5 | 0.495 |
| walker |  | 4656 | 154 | README.md section #1 |  |  | 0.495 |
| walker |  | 4782 | 126 | python decl names surface #1 in demo.py |  |  | 0.495 |
| walker |  | 4782 | 0 | python decl at demo.py:70 |  |  | 0.495 |
| walker |  | 4877 | 95 | python decl doc at demo.py:70 |  |  | 0.495 |
| ns | 4968 |  | 461 | BaseApi.login body — sends credentials, captures cookie | 5.8 | 5.6 | 0.469 |
| walker |  | 4978 | 101 | python class body at py3xui/inbound/sniffing.py:20 |  |  | 0.481 |
| walker |  | 5044 | 66 | python method at py3xui/api/api.py:139 |  |  | 0.500 |
| walker |  | 5111 | 67 | python method at py3xui/async_api/async_api.py:144 |  |  | 0.519 |
| walker |  | 5144 | 33 | python imports in py3xui/utils/env.py |  |  | 0.519 |
| walker |  | 5296 | 152 | README.md section #12 |  |  | 0.519 |
| walker |  | 5409 | 113 | python class body at py3xui/inbound/stream_settings.py:9 |  |  | 0.520 |
| walker |  | 5496 | 87 | python decl doc at py3xui/inbound/settings.py:17 |  |  | 0.520 |
| walker |  | 5633 | 137 | python method sigs in py3xui/api/api_inbound.py |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:40 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:71 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:111 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:159 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:190 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:223 |  |  | 0.547 |
| walker |  | 5633 | 0 | python method at py3xui/api/api_inbound.py:247 |  |  | 0.547 |
| walker |  | 5684 | 51 | README.md section #16 |  |  | 0.547 |
| walker |  | 5728 | 44 | python method doc at py3xui/api/api.py:93 |  |  | 0.547 |
| walker |  | 5772 | 44 | python method doc at py3xui/api/api.py:102 |  |  | 0.547 |
| ns | 5798 |  | 830 | _request_with_retry — TLS verify branch + retry/backoff body (sync) | 5.9 | 5.6 | 0.506 |
| walker |  | 5816 | 44 | python method doc at py3xui/async_api/async_api.py:98 |  |  | 0.506 |
| walker |  | 5860 | 44 | python method doc at py3xui/async_api/async_api.py:107 |  |  | 0.506 |
| walker |  | 6004 | 144 | python method sigs in py3xui/async_api/async_api_inbound.py |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:42 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:72 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:112 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:158 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:189 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:222 |  |  | 0.518 |
| walker |  | 6004 | 0 | python method at py3xui/async_api/async_api_inbound.py:246 |  |  | 0.518 |
| walker |  | 6041 | 37 | python imports in py3xui/server/server.py |  |  | 0.518 |
| walker |  | 6138 | 97 | python decl doc at py3xui/inbound/sniffing.py:20 |  |  | 0.518 |
| walker |  | 6176 | 38 | python imports in py3xui/client/client.py |  |  | 0.518 |
| walker |  | 6214 | 38 | python imports in py3xui/inbound/bases.py |  |  | 0.518 |
| ns | 6238 |  | 440 | _post and _get bodies — login gate + delegation to retry | 5.10 | 5.6 | 0.499 |
| walker |  | 6263 | 49 | python method doc at py3xui/inbound/inbound.py:114 |  |  | 0.499 |
| ns | 6395 |  | 157 | _check_response — success-field validation | 5.11 | 5.6 | 0.491 |
| walker |  | 6554 | 291 | LICENSE.md section #0 |  |  | 0.491 |
| walker |  | 6605 | 51 | python method doc at py3xui/api/api_server.py:97 |  |  | 0.491 |
| walker |  | 6656 | 51 | python method doc at py3xui/async_api/async_api.py:120 |  |  | 0.491 |
| walker |  | 6707 | 51 | python method doc at py3xui/async_api/async_api_server.py:105 |  |  | 0.491 |
| walker |  | 6790 | 83 | python decl names surface in py3xui/utils/env.py |  |  | 0.491 |
| walker |  | 6790 | 0 | python decl at py3xui/utils/env.py:33 |  |  | 0.491 |
| walker |  | 6790 | 0 | python decl at py3xui/utils/env.py:49 |  |  | 0.491 |
| walker |  | 6790 | 0 | python decl at py3xui/utils/env.py:65 |  |  | 0.491 |
| walker |  | 6790 | 0 | python decl at py3xui/utils/env.py:81 |  |  | 0.491 |
| walker |  | 6790 | 0 | python decl at py3xui/utils/env.py:95 |  |  | 0.491 |
| walker |  | 6856 | 66 | python decl at py3xui/utils/env.py:7 |  |  | 0.492 |
| walker |  | 6895 | 39 | python decl body at py3xui/utils/env.py:33 body 43 |  |  | 0.492 |
| walker |  | 6934 | 39 | python decl body at py3xui/utils/env.py:49 body 59 |  |  | 0.492 |
| walker |  | 6973 | 39 | python decl body at py3xui/utils/env.py:65 body 75 |  |  | 0.492 |
| walker |  | 7036 | 63 | python decl doc at py3xui/utils/env.py:81 |  |  | 0.492 |
| walker |  | 7101 | 65 | python decl doc at py3xui/utils/env.py:95 |  |  | 0.492 |
| walker |  | 7183 | 82 | python decl doc at py3xui/utils/env.py:33 |  |  | 0.492 |
| walker |  | 7265 | 82 | python decl doc at py3xui/utils/env.py:49 |  |  | 0.492 |
| walker |  | 7347 | 82 | python decl doc at py3xui/utils/env.py:65 |  |  | 0.492 |
| ns | 7374 |  | 979 | AsyncBaseApi _request_with_retry — httpx version | 5.12 | 5.3 | 0.459 |
| walker |  | 7397 | 50 | python decl body at py3xui/utils/env.py:95 body 102 |  |  | 0.459 |
| walker |  | 7452 | 55 | python method doc at py3xui/api/api.py:115 |  |  | 0.459 |
| ns | 7482 |  | 108 | utils/env public functions | 6.1 |  | 0.466 |
| walker |  | 7498 | 46 | python imports in py3xui/api/api_database.py |  |  | 0.466 |
| ns | 7632 |  | 150 | Api.from_env body — env var precedence + TLS defaults | 6.2 | 2.7 | 0.461 |
| walker |  | 7664 | 166 | python class body at py3xui/inbound/inbound.py:15 |  |  | 0.461 |
| walker |  | 7714 | 50 | python imports in py3xui/inbound/sniffing.py |  |  | 0.461 |
| walker |  | 7807 | 93 | python method at py3xui/api/api.py:67 |  |  | 0.477 |
| ns | 7866 |  | 234 | Api.login body — propagates session+cookie_name to sub-APIs | 6.3 | 2.7 | 0.468 |
| walker |  | 7900 | 93 | python method at py3xui/async_api/async_api.py:72 |  |  | 0.483 |
| walker |  | 7963 | 63 | python method doc at py3xui/api/api.py:124 |  |  | 0.483 |
| walker |  | 8026 | 63 | python method doc at py3xui/async_api/async_api.py:129 |  |  | 0.483 |
| walker |  | 8077 | 51 | python imports in py3xui/async_api/async_api_database.py |  |  | 0.483 |
| walker |  | 8129 | 52 | python imports in py3xui/inbound/stream_settings.py |  |  | 0.483 |
| ns | 8157 |  | 291 | Inbound.validate_stream_settings — dict/str/JSON handling | 6.5 | 4.2 | 0.474 |
| walker |  | 8194 | 65 | python method doc at py3xui/inbound/bases.py:12 |  |  | 0.482 |
| walker |  | 8301 | 107 | python decl names surface in py3xui/server/server.py |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:7 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:43 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:54 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:67 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:78 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:89 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:100 |  |  | 0.493 |
| walker |  | 8301 | 0 | python decl at py3xui/server/server.py:113 |  |  | 0.493 |
| walker |  | 8314 | 13 | python decl at py3xui/server/server.py:125 |  |  | 0.493 |
| walker |  | 8332 | 18 | python class body at py3xui/server/server.py:43 |  |  | 0.493 |
| walker |  | 8350 | 18 | python class body at py3xui/server/server.py:67 |  |  | 0.493 |
| walker |  | 8368 | 18 | python class body at py3xui/server/server.py:78 |  |  | 0.493 |
| walker |  | 8382 | 14 | python decl doc at py3xui/server/server.py:7 |  |  | 0.493 |
| walker |  | 8402 | 20 | python class body at py3xui/server/server.py:89 |  |  | 0.493 |
| walker |  | 8422 | 20 | python decl doc at py3xui/server/server.py:113 |  |  | 0.494 |
| walker |  | 8449 | 27 | python class body at py3xui/server/server.py:100 |  |  | 0.494 |
| walker |  | 8492 | 43 | python class body at py3xui/server/server.py:54 |  |  | 0.494 |
| walker |  | 8539 | 47 | python decl doc at py3xui/server/server.py:78 |  |  | 0.494 |
| ns | 8543 |  | 386 | Inbound.to_json — XUI API submit format | 6.6 | 4.2 | 0.481 |
| walker |  | 8588 | 49 | python decl doc at py3xui/server/server.py:67 |  |  | 0.481 |
| walker |  | 8640 | 52 | python decl doc at py3xui/server/server.py:43 |  |  | 0.481 |
| walker |  | 8694 | 54 | python decl doc at py3xui/server/server.py:89 |  |  | 0.481 |
| walker |  | 8755 | 61 | python decl doc at py3xui/server/server.py:100 |  |  | 0.481 |
| ns | 8801 |  | 258 | ClientFields constants roster — JSON-key alias mapping | 6.7 | 4.1 | 0.472 |
| walker |  | 8835 | 80 | python class body at py3xui/server/server.py:113 |  |  | 0.481 |
| walker |  | 8906 | 71 | python decl doc at py3xui/server/server.py:54 |  |  | 0.481 |
| walker |  | 8965 | 59 | README.md section #20 |  |  | 0.481 |
| ns | 9121 |  | 320 | SettingsFields, SniffingFields, StreamSettingsFields rosters | 6.9 | 4.4 | 0.493 |
| walker |  | 9180 | 215 | python method sigs in py3xui/api/api_client.py |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:52 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:89 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:121 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:157 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:189 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:217 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:248 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:280 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:312 |  |  | 0.516 |
| walker |  | 9180 | 0 | python method at py3xui/api/api_client.py:342 |  |  | 0.516 |
| ns | 9180 |  | 59 | Test file roster + tests/responses fixtures | 6.10 |  | 0.516 |
| walker |  | 9272 | 92 | py3xui/server/README.md section #2 |  |  | 0.516 |
| walker |  | 9333 | 61 | README.md section #18 |  |  | 0.516 |
| walker |  | 9558 | 225 | python method sigs in py3xui/async_api/async_api_client.py |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:52 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:89 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:121 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:158 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:190 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:219 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:250 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:282 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:315 |  |  | 0.528 |
| walker |  | 9558 | 0 | python method at py3xui/async_api/async_api_client.py:345 |  |  | 0.528 |
| ns | 9571 |  | 391 | Sync API endpoint URLs — catalog grouped by sub-API | 6.11 | 3.8 | 0.520 |
| walker |  | 9624 | 66 | python decl body at py3xui/utils/env.py:81 body 88 |  |  | 0.520 |
| walker |  | 9682 | 58 | python imports in py3xui/api/api_server.py |  |  | 0.520 |
| walker |  | 9879 | 197 | python class body at py3xui/client/client.py:7 |  |  | 0.539 |
| walker |  | 9887 | 8 | python method body at py3xui/inbound/bases.py:12 body 30 |  |  | 0.541 |
| ns | 9905 |  | 334 | README quickstart — env vars + sync/async construction | 6.12 |  | 0.530 |
| walker |  | 9950 | 63 | python imports in py3xui/inbound/settings.py |  |  | 0.530 |

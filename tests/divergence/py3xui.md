Score(3000)=0.621 I=0.857 C=0.450 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.861/0.847/0.720/0.621/0.588/0.570/0.595

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 74 |  | 74 | What py3xui is: distribution name, version, one-line purpose | 1.1 |  | 0.000 |
| walker |  | 81 | 25 | Fs::DirListing { dir: py3xui } |  |  | 0.000 |
| walker |  | 95 | 14 | Fs::DirListing { dir: py3xui/client } |  |  | 0.000 |
| walker |  | 109 | 14 | Fs::DirListing { dir: py3xui/server } |  |  | 0.000 |
| walker |  | 123 | 14 | Fs::DirListing { dir: py3xui/utils } |  |  | 0.000 |
| ns | 130 |  | 56 | Complete repository root listing | 1.2 |  | 0.628 |
| walker |  | 150 | 27 | Fs::DirListing { dir: dev } |  |  | 0.628 |
| walker |  | 182 | 32 | Fs::DirListing { dir: py3xui/inbound } |  |  | 0.651 |
| ns | 212 |  | 82 | Top-level public exports: py3xui/__init__.py in full | 1.3 |  | 0.562 |
| walker |  | 222 | 40 | Fs::DirListing { dir: py3xui/api } |  |  | 0.571 |
| ns | 237 |  | 25 | py3xui/ package listing — the six sub-packages | 1.4 |  | 0.589 |
| walker |  | 268 | 46 | Fs::DirListing { dir: py3xui/async_api } |  |  | 0.614 |
| walker |  | 339 | 71 | Toml::Identity { file: pyproject.toml } |  |  | 0.738 |
| walker |  | 351 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.738 |
| walker |  | 369 | 18 | Fs::DirListing { dir: .github } |  |  | 0.738 |
| ns | 377 |  | 140 | README Overview: purpose and runtime dependencies | 1.5 |  | 0.662 |
| walker |  | 392 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.662 |
| walker |  | 580 | 188 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.705 |
| ns | 590 |  | 213 | Every remaining README section heading (complete map of the root README) | 1.6 |  | 0.559 |
| walker |  | 639 | 59 | Markdown::Prelude { file: README.md } |  |  | 0.705 |
| walker |  | 666 | 27 | Fs::DirListing { dir: tests } |  |  | 0.705 |
| ns | 709 |  | 119 | Supported Python versions and 3x-ui compatibility floor | 1.7 |  | 0.662 |
| walker |  | 725 | 59 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.662 |
| walker |  | 758 | 33 | Markdown::HeadingsOutline { file: py3xui/client/README.md } |  |  | 0.662 |
| ns | 795 |  | 86 | Listings of the two transport packages: py3xui/api/ and py3xui/async_api/ | 1.8 |  | 0.717 |
| ns | 869 |  | 74 | Listings of the model and utility packages: inbound/, client/, server/, utils/ | 1.9 |  | 0.746 |
| walker |  | 925 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.829 |
| ns | 1016 |  | 147 | Re-export blocks of py3xui/api/__init__.py and py3xui/async_api/__init__.py | 1.10 |  | 0.789 |
| walker |  | 1087 | 162 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.859 |
| walker |  | 1130 | 43 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.859 |
| walker |  | 1146 | 16 | Code::CodeKey { rung: Names, file: py3xui/client/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.859 |
| ns | 1157 |  | 141 | Re-export blocks of the inbound/, client/, server/ and utils/ packages | 1.11 |  | 0.817 |
| walker |  | 1228 | 82 | Code::CodeKey { rung: Names, file: py3xui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.862 |
| walker |  | 1296 | 68 | Code::CodeKey { rung: Names, file: py3xui/api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.872 |
| walker |  | 1331 | 35 | Code::CodeKey { rung: Names, file: py3xui/utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.878 |
| ns | 1342 |  | 185 | Canonical usage: env-var credentials, Api.from_env() and AsyncApi.from_env() | 2.1 |  | 0.817 |
| walker |  | 1401 | 70 | Code::CodeKey { rung: Names, file: py3xui/inbound/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.847 |
| ns | 1459 |  | 117 | class Api and its full constructor signature | 2.2 |  | 0.805 |
| ns | 1665 |  | 206 | Api constructor body: the four sub-API attributes it wires up | 2.3 | 2.2 | 0.753 |
| walker |  | 1749 | 348 | Code::CodeKey { rung: Names, file: demo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 1828 | 79 | Code::CodeKey { rung: Names, file: py3xui/async_api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.783 |
| ns | 1840 |  | 175 | class AsyncApi: identical constructor, Async* sub-APIs | 2.4 |  | 0.740 |
| walker |  | 1848 | 20 | Code::CodeKey { rung: Names, file: py3xui/server/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 1955 | 107 | Code::CodeKey { rung: Doc, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.750 |
| ns | 2063 |  | 223 | ClientApi methods 1-5 with the panel endpoint each one calls | 2.5 |  | 0.720 |
| walker |  | 2109 | 154 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.720 |
| walker |  | 2221 | 112 | Markdown::HeadingsOutline { file: py3xui/server/README.md } |  |  | 0.720 |
| ns | 2292 |  | 229 | ClientApi methods 6-10 with their endpoints (closes the ClientApi roster) | 2.6 | 2.5 | 0.697 |
| walker |  | 2399 | 178 | Code::CodeKey { rung: Body, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.697 |
| walker |  | 2557 | 158 | Markdown::HeadingsOutline { file: py3xui/api/README.md } |  |  | 0.697 |
| ns | 2586 |  | 294 | InboundApi: complete method roster with endpoints | 2.7 |  | 0.664 |
| walker |  | 2720 | 163 | Markdown::HeadingsOutline { file: py3xui/inbound/README.md } |  |  | 0.664 |
| ns | 2750 |  | 164 | ServerApi and DatabaseApi: complete rosters with endpoints | 2.8 |  | 0.642 |
| walker |  | 2762 | 42 | Markdown::Section { file: py3xui/server/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 2805 | 43 | Markdown::Section { file: py3xui/client/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.642 |
| ns | 2926 |  | 176 | Api.from_env signature and the complete environment-variable contract | 2.9 |  | 0.621 |
| walker |  | 2990 | 185 | Markdown::HeadingsOutline { file: py3xui/async_api/README.md } |  |  | 0.621 |
| ns | 3158 |  | 232 | from_env body and Api.login: the two-step connect sequence | 2.10 | 2.9 | 0.594 |
| walker |  | 3227 | 237 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.621 |
| walker |  | 3249 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 3327 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.622 |
| walker |  | 3343 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.622 |
| walker |  | 3354 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 3400 |  | 242 | AsyncClientApi: complete async method roster | 2.11 |  | 0.605 |
| walker |  | 3474 | 120 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.606 |
| walker |  | 3482 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.606 |
| walker |  | 3490 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.606 |
| walker |  | 3499 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.606 |
| walker |  | 3509 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 6, sub: 0, line: 124 } |  |  | 0.606 |
| walker |  | 3576 | 67 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.614 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.594 |
| walker |  | 3671 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.623 |
| walker |  | 3681 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.623 |
| walker |  | 3728 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.623 |
| walker |  | 3777 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.623 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.613 |
| walker |  | 3828 | 51 | Markdown::Section { file: py3xui/utils/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.613 |
| walker |  | 3851 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.607 |
| walker |  | 4078 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.610 |
| walker |  | 4094 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.611 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.606 |
| walker |  | 4106 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4227 | 121 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.607 |
| walker |  | 4235 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.607 |
| walker |  | 4243 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.607 |
| walker |  | 4252 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.607 |
| walker |  | 4262 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 6, sub: 0, line: 129 } |  |  | 0.607 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.588 |
| walker |  | 4330 | 68 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.588 |
| walker |  | 4425 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.605 |
| walker |  | 4435 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.605 |
| walker |  | 4482 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.605 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.592 |
| walker |  | 4531 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.592 |
| walker |  | 4555 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 4590 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.592 |
| walker |  | 4627 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.592 |
| walker |  | 4643 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.592 |
| walker |  | 4699 | 56 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.592 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.618 |
| walker |  | 4808 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 4826 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.619 |
| walker |  | 4844 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.619 |
| walker |  | 4862 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.619 |
| walker |  | 4882 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.619 |
| walker |  | 4909 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.620 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.606 |
| walker |  | 4952 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.606 |
| walker |  | 5037 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.607 |
| walker |  | 5057 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.607 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.593 |
| walker |  | 5109 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.593 |
| walker |  | 5163 | 54 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.593 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.575 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.569 |
| walker |  | 5503 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.571 |
| walker |  | 5560 | 57 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.571 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.550 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.540 |
| walker |  | 5909 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.540 |
| walker |  | 5923 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.540 |
| walker |  | 5948 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.531 |
| walker |  | 6129 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.559 |
| walker |  | 6145 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.566 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.558 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.550 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.541 |
| walker |  | 6486 | 341 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.588 |
| walker |  | 6522 | 36 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.590 |
| walker |  | 6578 | 56 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 4, sub: 0, line: 114 } |  |  | 0.599 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.613 |
| walker |  | 6935 | 357 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.615 |
| walker |  | 6943 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.615 |
| walker |  | 6951 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.615 |
| walker |  | 6959 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.615 |
| walker |  | 6967 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.615 |
| walker |  | 6975 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.615 |
| walker |  | 6983 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.615 |
| walker |  | 6991 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.615 |
| walker |  | 6999 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.615 |
| walker |  | 7007 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.615 |
| walker |  | 7016 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.615 |
| walker |  | 7026 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.615 |
| walker |  | 7037 | 11 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.615 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.627 |
| walker |  | 7075 | 38 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.627 |
| walker |  | 7135 | 60 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.627 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.632 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.633 |
| walker |  | 7230 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.634 |
| walker |  | 7271 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.634 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.638 |
| walker |  | 7312 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.638 |
| walker |  | 7353 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.638 |
| walker |  | 7396 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.638 |
| walker |  | 7439 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.638 |
| walker |  | 7482 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.638 |
| walker |  | 7527 | 45 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.638 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.631 |
| walker |  | 7573 | 46 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.631 |
| walker |  | 7621 | 48 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.631 |
| walker |  | 7670 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.631 |
| walker |  | 7722 | 52 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.631 |
| walker |  | 7779 | 57 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.631 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.640 |
| walker |  | 7838 | 59 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.640 |
| walker |  | 7898 | 60 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.640 |
| walker |  | 7911 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.631 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.620 |
| walker |  | 8277 | 366 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.620 |
| walker |  | 8285 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.620 |
| walker |  | 8293 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.620 |
| walker |  | 8301 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.620 |
| walker |  | 8309 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.620 |
| walker |  | 8317 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.620 |
| walker |  | 8325 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.620 |
| walker |  | 8333 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.620 |
| walker |  | 8341 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.620 |
| walker |  | 8349 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.620 |
| walker |  | 8358 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.620 |
| walker |  | 8368 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.620 |
| walker |  | 8379 | 11 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.620 |
| walker |  | 8418 | 39 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 20, sub: 0, line: 308 } |  |  | 0.620 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.611 |
| walker |  | 8476 | 58 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 15, sub: 0, line: 166 } |  |  | 0.611 |
| walker |  | 8571 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 2, sub: 0, line: 49 } |  |  | 0.611 |
| walker |  | 8612 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.611 |
| walker |  | 8653 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.611 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.604 |
| walker |  | 8694 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.604 |
| walker |  | 8737 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.604 |
| walker |  | 8780 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.604 |
| walker |  | 8823 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.604 |
| walker |  | 8868 | 45 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.604 |
| walker |  | 8914 | 46 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.604 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.595 |
| walker |  | 8962 | 48 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.595 |
| walker |  | 9011 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.595 |
| walker |  | 9063 | 52 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.595 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.590 |
| walker |  | 9120 | 57 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.590 |
| walker |  | 9146 | 26 | Code::CodeKey { rung: Names, file: py3xui/inbound/stream_settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.593 |
| walker |  | 9269 | 123 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.604 |
| walker |  | 9285 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.604 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.596 |
| walker |  | 9617 | 332 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 2, sub: 0, line: 25 } |  |  | 0.608 |
| walker |  | 9631 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.596 |
| walker |  | 9846 | 215 | Code::CodeKey { rung: Decl, file: py3xui/api/api_client.py, decl: 1, sub: 0, line: 13 } |  |  | 0.602 |
| walker |  | 9860 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_database.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 9874 | 14 | Code::CodeKey { rung: Decl, file: py3xui/api/api_database.py, decl: 1, sub: 0, line: 7 } |  |  | 0.602 |
| walker |  | 9888 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.594 |
| walker |  | 9942 | 54 | Code::CodeKey { rung: Decl, file: py3xui/api/api_server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.596 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.595 |
| walker |  | 10000 | 58 | Code::CodeKey { rung: Doc, file: py3xui/api/api_server.py, decl: 4, sub: 0, line: 97 } |  |  | 0.595 |

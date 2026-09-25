Score(3000)=0.621 I=0.857 C=0.450 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.861/0.836/0.720/0.621/0.582/0.581/0.586

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
| walker |  | 1142 | 55 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.859 |
| ns | 1157 |  | 141 | Re-export blocks of the inbound/, client/, server/ and utils/ packages | 1.11 |  | 0.816 |
| walker |  | 1185 | 43 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.816 |
| walker |  | 1201 | 16 | Code::CodeKey { rung: Names, file: py3xui/client/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| walker |  | 1283 | 82 | Code::CodeKey { rung: Names, file: py3xui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.862 |
| ns | 1342 |  | 185 | Canonical usage: env-var credentials, Api.from_env() and AsyncApi.from_env() | 2.1 |  | 0.802 |
| walker |  | 1351 | 68 | Code::CodeKey { rung: Names, file: py3xui/api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| walker |  | 1386 | 35 | Code::CodeKey { rung: Names, file: py3xui/utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| walker |  | 1456 | 70 | Code::CodeKey { rung: Names, file: py3xui/inbound/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.847 |
| ns | 1459 |  | 117 | class Api and its full constructor signature | 2.2 |  | 0.805 |
| ns | 1665 |  | 206 | Api constructor body: the four sub-API attributes it wires up | 2.3 | 2.2 | 0.753 |
| walker |  | 1804 | 348 | Code::CodeKey { rung: Names, file: demo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| ns | 1840 |  | 175 | class AsyncApi: identical constructor, Async* sub-APIs | 2.4 |  | 0.712 |
| walker |  | 1883 | 79 | Code::CodeKey { rung: Names, file: py3xui/async_api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.740 |
| walker |  | 1903 | 20 | Code::CodeKey { rung: Names, file: py3xui/server/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 2010 | 107 | Code::CodeKey { rung: Doc, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.750 |
| ns | 2063 |  | 223 | ClientApi methods 1-5 with the panel endpoint each one calls | 2.5 |  | 0.720 |
| walker |  | 2122 | 112 | Markdown::HeadingsOutline { file: py3xui/server/README.md } |  |  | 0.720 |
| ns | 2292 |  | 229 | ClientApi methods 6-10 with their endpoints (closes the ClientApi roster) | 2.6 | 2.5 | 0.697 |
| walker |  | 2300 | 178 | Code::CodeKey { rung: Body, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.697 |
| walker |  | 2458 | 158 | Markdown::HeadingsOutline { file: py3xui/api/README.md } |  |  | 0.697 |
| ns | 2586 |  | 294 | InboundApi: complete method roster with endpoints | 2.7 |  | 0.664 |
| walker |  | 2621 | 163 | Markdown::HeadingsOutline { file: py3xui/inbound/README.md } |  |  | 0.664 |
| walker |  | 2663 | 42 | Markdown::Section { file: py3xui/server/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.664 |
| walker |  | 2706 | 43 | Markdown::Section { file: py3xui/client/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.664 |
| ns | 2750 |  | 164 | ServerApi and DatabaseApi: complete rosters with endpoints | 2.8 |  | 0.642 |
| walker |  | 2891 | 185 | Markdown::HeadingsOutline { file: py3xui/async_api/README.md } |  |  | 0.642 |
| ns | 2926 |  | 176 | Api.from_env signature and the complete environment-variable contract | 2.9 |  | 0.621 |
| walker |  | 3045 | 154 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.621 |
| walker |  | 3067 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 3145 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.622 |
| ns | 3158 |  | 232 | from_env body and Api.login: the two-step connect sequence | 2.10 | 2.9 | 0.595 |
| walker |  | 3161 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.595 |
| walker |  | 3172 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3292 | 120 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.596 |
| walker |  | 3300 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.596 |
| walker |  | 3308 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.596 |
| walker |  | 3317 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.596 |
| walker |  | 3327 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 6, sub: 0, line: 124 } |  |  | 0.596 |
| walker |  | 3394 | 67 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.604 |
| ns | 3400 |  | 242 | AsyncClientApi: complete async method roster | 2.11 |  | 0.588 |
| walker |  | 3489 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.619 |
| walker |  | 3499 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.619 |
| walker |  | 3546 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.619 |
| walker |  | 3595 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.619 |
| walker |  | 3646 | 51 | Markdown::Section { file: py3xui/utils/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.619 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.599 |
| walker |  | 3669 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.589 |
| walker |  | 3896 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.592 |
| walker |  | 3912 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.593 |
| walker |  | 3924 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.587 |
| walker |  | 4045 | 121 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.587 |
| walker |  | 4053 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.587 |
| walker |  | 4061 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.587 |
| walker |  | 4070 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.587 |
| walker |  | 4080 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 6, sub: 0, line: 129 } |  |  | 0.587 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.583 |
| walker |  | 4148 | 68 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.583 |
| walker |  | 4243 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.600 |
| walker |  | 4253 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.600 |
| walker |  | 4300 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.600 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.582 |
| walker |  | 4349 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.582 |
| walker |  | 4373 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 4408 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.582 |
| walker |  | 4445 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.582 |
| walker |  | 4461 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.582 |
| walker |  | 4517 | 56 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.582 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.569 |
| walker |  | 4626 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4644 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.570 |
| walker |  | 4662 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.570 |
| walker |  | 4680 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.570 |
| walker |  | 4700 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.570 |
| walker |  | 4727 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.571 |
| walker |  | 4770 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.571 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.600 |
| walker |  | 4855 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.601 |
| walker |  | 4875 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.601 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.588 |
| walker |  | 4927 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.588 |
| walker |  | 4981 | 54 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.588 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.574 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.556 |
| walker |  | 5321 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.558 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.553 |
| walker |  | 5378 | 57 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.553 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.532 |
| walker |  | 5727 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.532 |
| walker |  | 5741 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.533 |
| walker |  | 5766 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.523 |
| walker |  | 5947 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.553 |
| walker |  | 5963 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.560 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.549 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.541 |
| walker |  | 6304 | 341 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.591 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.582 |
| walker |  | 6340 | 36 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.584 |
| walker |  | 6396 | 56 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 4, sub: 0, line: 114 } |  |  | 0.594 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.584 |
| walker |  | 6753 | 357 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.586 |
| walker |  | 6761 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.586 |
| walker |  | 6769 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.586 |
| walker |  | 6777 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.586 |
| walker |  | 6785 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.586 |
| walker |  | 6793 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.586 |
| walker |  | 6801 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.586 |
| walker |  | 6809 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.586 |
| walker |  | 6817 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.586 |
| walker |  | 6825 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.586 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.600 |
| walker |  | 6834 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.600 |
| walker |  | 6844 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.600 |
| walker |  | 6855 | 11 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.600 |
| walker |  | 6893 | 38 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.600 |
| walker |  | 6953 | 60 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.600 |
| walker |  | 7048 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.601 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.614 |
| walker |  | 7089 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.614 |
| walker |  | 7130 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.614 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.619 |
| walker |  | 7171 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.619 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.620 |
| walker |  | 7214 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.620 |
| walker |  | 7257 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.620 |
| walker |  | 7300 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.620 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.625 |
| walker |  | 7345 | 45 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.625 |
| walker |  | 7391 | 46 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.625 |
| walker |  | 7439 | 48 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.625 |
| walker |  | 7488 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.625 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.618 |
| walker |  | 7540 | 52 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.618 |
| walker |  | 7597 | 57 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.618 |
| walker |  | 7656 | 59 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.618 |
| walker |  | 7716 | 60 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.618 |
| walker |  | 7729 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.628 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.618 |
| walker |  | 8095 | 366 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.618 |
| walker |  | 8103 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.618 |
| walker |  | 8111 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.618 |
| walker |  | 8119 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.618 |
| walker |  | 8127 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.618 |
| walker |  | 8135 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.618 |
| walker |  | 8143 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.618 |
| walker |  | 8151 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.618 |
| walker |  | 8159 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.618 |
| walker |  | 8167 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.618 |
| walker |  | 8176 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.618 |
| walker |  | 8186 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.618 |
| walker |  | 8197 | 11 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.618 |
| walker |  | 8236 | 39 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 20, sub: 0, line: 308 } |  |  | 0.618 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.608 |
| walker |  | 8294 | 58 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 15, sub: 0, line: 166 } |  |  | 0.608 |
| walker |  | 8389 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 2, sub: 0, line: 49 } |  |  | 0.608 |
| walker |  | 8430 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.608 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.599 |
| walker |  | 8471 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.599 |
| walker |  | 8512 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.599 |
| walker |  | 8555 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.599 |
| walker |  | 8598 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.599 |
| walker |  | 8641 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.599 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.592 |
| walker |  | 8686 | 45 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.592 |
| walker |  | 8732 | 46 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.592 |
| walker |  | 8780 | 48 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.592 |
| walker |  | 8829 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.592 |
| walker |  | 8881 | 52 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.592 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.583 |
| walker |  | 8938 | 57 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.583 |
| walker |  | 8964 | 26 | Code::CodeKey { rung: Names, file: py3xui/inbound/stream_settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.579 |
| walker |  | 9087 | 123 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.591 |
| walker |  | 9103 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.591 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.593 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.585 |
| walker |  | 9435 | 332 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 2, sub: 0, line: 25 } |  |  | 0.597 |
| walker |  | 9449 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 9664 | 215 | Code::CodeKey { rung: Decl, file: py3xui/api/api_client.py, decl: 1, sub: 0, line: 13 } |  |  | 0.603 |
| walker |  | 9678 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_database.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 9692 | 14 | Code::CodeKey { rung: Decl, file: py3xui/api/api_database.py, decl: 1, sub: 0, line: 7 } |  |  | 0.603 |
| walker |  | 9706 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.592 |
| walker |  | 9760 | 54 | Code::CodeKey { rung: Decl, file: py3xui/api/api_server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.595 |
| walker |  | 9818 | 58 | Code::CodeKey { rung: Doc, file: py3xui/api/api_server.py, decl: 4, sub: 0, line: 97 } |  |  | 0.595 |
| walker |  | 9846 | 28 | Code::CodeKey { rung: Names, file: py3xui/inbound/sniffing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 9905 | 59 | Code::CodeKey { rung: Decl, file: py3xui/inbound/sniffing.py, decl: 1, sub: 0, line: 9 } |  |  | 0.600 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.590 |
| walker |  | 9921 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/sniffing.py, decl: 1, sub: 0, line: 9 } |  |  | 0.590 |
| walker |  | 9973 | 52 | Code::CodeKey { rung: Decl, file: py3xui/inbound/sniffing.py, decl: 2, sub: 0, line: 20 } |  |  | 0.594 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.592 |

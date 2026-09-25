Score(3000)=0.621 I=0.857 C=0.450 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.861/0.836/0.720/0.621/0.566/0.543/0.584

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
| walker |  | 2253 | 131 | Toml::Config { file: pyproject.toml } |  |  | 0.720 |
| ns | 2292 |  | 229 | ClientApi methods 6-10 with their endpoints (closes the ClientApi roster) | 2.6 | 2.5 | 0.697 |
| walker |  | 2431 | 178 | Code::CodeKey { rung: Body, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.697 |
| ns | 2586 |  | 294 | InboundApi: complete method roster with endpoints | 2.7 |  | 0.664 |
| walker |  | 2589 | 158 | Markdown::HeadingsOutline { file: py3xui/api/README.md } |  |  | 0.664 |
| ns | 2750 |  | 164 | ServerApi and DatabaseApi: complete rosters with endpoints | 2.8 |  | 0.642 |
| walker |  | 2752 | 163 | Markdown::HeadingsOutline { file: py3xui/inbound/README.md } |  |  | 0.642 |
| walker |  | 2794 | 42 | Markdown::Section { file: py3xui/server/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 2837 | 43 | Markdown::Section { file: py3xui/client/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.642 |
| ns | 2926 |  | 176 | Api.from_env signature and the complete environment-variable contract | 2.9 |  | 0.621 |
| walker |  | 3022 | 185 | Markdown::HeadingsOutline { file: py3xui/async_api/README.md } |  |  | 0.621 |
| ns | 3158 |  | 232 | from_env body and Api.login: the two-step connect sequence | 2.10 | 2.9 | 0.594 |
| walker |  | 3176 | 154 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 3299 | 123 | Plaintext::Whole { file: dev/requirements.txt } |  |  | 0.595 |
| walker |  | 3321 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3399 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.595 |
| ns | 3400 |  | 242 | AsyncClientApi: complete async method roster | 2.11 |  | 0.579 |
| walker |  | 3415 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.579 |
| walker |  | 3426 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 3546 | 120 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.580 |
| walker |  | 3554 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.580 |
| walker |  | 3562 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.580 |
| walker |  | 3571 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.580 |
| walker |  | 3581 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 6, sub: 0, line: 124 } |  |  | 0.580 |
| walker |  | 3648 | 67 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.588 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.569 |
| walker |  | 3743 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.599 |
| walker |  | 3753 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.599 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.589 |
| walker |  | 3800 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.589 |
| walker |  | 3849 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.589 |
| walker |  | 3900 | 51 | Markdown::Section { file: py3xui/utils/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 3923 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.583 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.579 |
| walker |  | 4150 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.582 |
| walker |  | 4166 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.583 |
| walker |  | 4178 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 4299 | 121 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.583 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.566 |
| walker |  | 4307 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.566 |
| walker |  | 4315 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.566 |
| walker |  | 4324 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.566 |
| walker |  | 4334 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 6, sub: 0, line: 129 } |  |  | 0.566 |
| walker |  | 4402 | 68 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.566 |
| walker |  | 4497 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.583 |
| walker |  | 4507 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.583 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.570 |
| walker |  | 4554 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.570 |
| walker |  | 4603 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.570 |
| walker |  | 4627 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4662 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.570 |
| walker |  | 4699 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.570 |
| walker |  | 4715 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.570 |
| walker |  | 4771 | 56 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.570 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.599 |
| walker |  | 4880 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 4898 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.599 |
| walker |  | 4916 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.600 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.586 |
| walker |  | 4934 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.586 |
| walker |  | 4954 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.586 |
| walker |  | 4981 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.587 |
| walker |  | 5024 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.587 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.574 |
| walker |  | 5109 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.574 |
| walker |  | 5129 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.574 |
| walker |  | 5181 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.574 |
| walker |  | 5235 | 54 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.574 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.557 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.551 |
| walker |  | 5575 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.553 |
| walker |  | 5632 | 57 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.553 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.533 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.523 |
| walker |  | 5981 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.523 |
| walker |  | 5995 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.523 |
| walker |  | 6020 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.514 |
| walker |  | 6201 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.543 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.535 |
| walker |  | 6217 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.542 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.534 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.525 |
| walker |  | 6558 | 341 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.573 |
| walker |  | 6594 | 36 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.575 |
| walker |  | 6650 | 56 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 4, sub: 0, line: 114 } |  |  | 0.584 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.599 |
| walker |  | 7007 | 357 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.601 |
| walker |  | 7015 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.601 |
| walker |  | 7023 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.601 |
| walker |  | 7031 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.601 |
| walker |  | 7039 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.601 |
| walker |  | 7047 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.601 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.614 |
| walker |  | 7055 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.614 |
| walker |  | 7063 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.614 |
| walker |  | 7071 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.614 |
| walker |  | 7079 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.614 |
| walker |  | 7088 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.614 |
| walker |  | 7098 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.614 |
| walker |  | 7109 | 11 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.614 |
| walker |  | 7147 | 38 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.614 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.619 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.621 |
| walker |  | 7207 | 60 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.621 |
| walker |  | 7302 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.621 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.626 |
| walker |  | 7343 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.626 |
| walker |  | 7384 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.626 |
| walker |  | 7425 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.626 |
| walker |  | 7468 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.626 |
| walker |  | 7511 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.626 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.619 |
| walker |  | 7554 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.619 |
| walker |  | 7599 | 45 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.619 |
| walker |  | 7645 | 46 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.619 |
| walker |  | 7693 | 48 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.619 |
| walker |  | 7742 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.619 |
| walker |  | 7794 | 52 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.619 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.628 |
| walker |  | 7851 | 57 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.628 |
| walker |  | 7910 | 59 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.628 |
| walker |  | 7970 | 60 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.628 |
| walker |  | 7983 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.619 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.609 |
| walker |  | 8349 | 366 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.609 |
| walker |  | 8357 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.609 |
| walker |  | 8365 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.609 |
| walker |  | 8373 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.609 |
| walker |  | 8381 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.609 |
| walker |  | 8389 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.609 |
| walker |  | 8397 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.609 |
| walker |  | 8405 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.609 |
| walker |  | 8413 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.609 |
| walker |  | 8421 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.609 |
| walker |  | 8430 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.609 |
| walker |  | 8440 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.609 |
| walker |  | 8451 | 11 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.609 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.600 |
| walker |  | 8490 | 39 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 20, sub: 0, line: 308 } |  |  | 0.600 |
| walker |  | 8548 | 58 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 15, sub: 0, line: 166 } |  |  | 0.600 |
| walker |  | 8643 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 2, sub: 0, line: 49 } |  |  | 0.600 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.592 |
| walker |  | 8684 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.592 |
| walker |  | 8725 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.592 |
| walker |  | 8766 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.592 |
| walker |  | 8809 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.592 |
| walker |  | 8852 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.592 |
| walker |  | 8895 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.592 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.584 |
| walker |  | 8940 | 45 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.584 |
| walker |  | 8986 | 46 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.584 |
| walker |  | 9034 | 48 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.584 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.579 |
| walker |  | 9083 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.579 |
| walker |  | 9135 | 52 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.579 |
| walker |  | 9192 | 57 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.579 |
| walker |  | 9218 | 26 | Code::CodeKey { rung: Names, file: py3xui/inbound/stream_settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.582 |
| walker |  | 9341 | 123 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.593 |
| walker |  | 9357 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.593 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.585 |
| walker |  | 9689 | 332 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 2, sub: 0, line: 25 } |  |  | 0.598 |
| walker |  | 9703 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.586 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.584 |
| walker |  | 9918 | 215 | Code::CodeKey { rung: Decl, file: py3xui/api/api_client.py, decl: 1, sub: 0, line: 13 } |  |  | 0.590 |
| walker |  | 9932 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_database.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 9946 | 14 | Code::CodeKey { rung: Decl, file: py3xui/api/api_database.py, decl: 1, sub: 0, line: 7 } |  |  | 0.590 |
| walker |  | 9960 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.589 |
| walker |  | 9995 | 35 | Code::CodeKey { rung: Decl, file: py3xui/api/api_server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.591 |

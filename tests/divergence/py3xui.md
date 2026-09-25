Score(3000)=0.621 I=0.857 C=0.450 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.746/0.698/0.683/0.621/0.548/0.520/0.593

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
| ns | 377 |  | 140 | README Overview: purpose and runtime dependencies | 1.5 |  | 0.661 |
| walker |  | 410 | 59 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.661 |
| walker |  | 428 | 18 | Fs::DirListing { dir: .github } |  |  | 0.662 |
| walker |  | 451 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.662 |
| ns | 590 |  | 213 | Every remaining README section heading (complete map of the root README) | 1.6 |  | 0.525 |
| walker |  | 639 | 188 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.559 |
| walker |  | 698 | 59 | Markdown::Prelude { file: README.md } |  |  | 0.705 |
| ns | 709 |  | 119 | Supported Python versions and 3x-ui compatibility floor | 1.7 |  | 0.661 |
| walker |  | 725 | 27 | Fs::DirListing { dir: tests } |  |  | 0.662 |
| walker |  | 780 | 55 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.662 |
| ns | 795 |  | 86 | Listings of the two transport packages: py3xui/api/ and py3xui/async_api/ | 1.8 |  | 0.717 |
| walker |  | 844 | 64 | Markdown::ReadmeHeadline { file: py3xui/inbound/README.md } |  |  | 0.717 |
| ns | 869 |  | 74 | Listings of the model and utility packages: inbound/, client/, server/, utils/ | 1.9 |  | 0.746 |
| walker |  | 910 | 66 | Markdown::ReadmeHeadline { file: py3xui/api/README.md } |  |  | 0.746 |
| walker |  | 977 | 67 | Markdown::ReadmeHeadline { file: py3xui/utils/README.md } |  |  | 0.746 |
| ns | 1016 |  | 147 | Re-export blocks of py3xui/api/__init__.py and py3xui/async_api/__init__.py | 1.10 |  | 0.710 |
| walker |  | 1049 | 72 | Markdown::ReadmeHeadline { file: py3xui/server/README.md } |  |  | 0.710 |
| walker |  | 1122 | 73 | Markdown::ReadmeHeadline { file: py3xui/client/README.md } |  |  | 0.710 |
| walker |  | 1141 | 19 | Markdown::HeadingsOutline { file: py3xui/client/README.md } |  |  | 0.710 |
| ns | 1157 |  | 141 | Re-export blocks of the inbound/, client/, server/ and utils/ packages | 1.11 |  | 0.674 |
| walker |  | 1220 | 79 | Markdown::ReadmeHeadline { file: py3xui/async_api/README.md } |  |  | 0.674 |
| ns | 1342 |  | 185 | Canonical usage: env-var credentials, Api.from_env() and AsyncApi.from_env() | 2.1 |  | 0.628 |
| walker |  | 1387 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.698 |
| ns | 1459 |  | 117 | class Api and its full constructor signature | 2.2 |  | 0.663 |
| walker |  | 1549 | 162 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.722 |
| walker |  | 1592 | 43 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.722 |
| ns | 1665 |  | 206 | Api constructor body: the four sub-API attributes it wires up | 2.3 | 2.2 | 0.676 |
| walker |  | 1723 | 131 | Toml::Config { file: pyproject.toml } |  |  | 0.676 |
| walker |  | 1739 | 16 | Code::CodeKey { rung: Names, file: py3xui/client/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 1821 | 82 | Code::CodeKey { rung: Names, file: py3xui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| ns | 1840 |  | 175 | class AsyncApi: identical constructor, Async* sub-APIs | 2.4 |  | 0.674 |
| walker |  | 1889 | 68 | Code::CodeKey { rung: Names, file: py3xui/api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 1924 | 35 | Code::CodeKey { rung: Names, file: py3xui/utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 1994 | 70 | Code::CodeKey { rung: Names, file: py3xui/inbound/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| ns | 2063 |  | 223 | ClientApi methods 1-5 with the panel endpoint each one calls | 2.5 |  | 0.683 |
| ns | 2292 |  | 229 | ClientApi methods 6-10 with their endpoints (closes the ClientApi roster) | 2.6 | 2.5 | 0.662 |
| walker |  | 2342 | 348 | Code::CodeKey { rung: Names, file: demo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 2421 | 79 | Code::CodeKey { rung: Names, file: py3xui/async_api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 2441 | 20 | Code::CodeKey { rung: Names, file: py3xui/server/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 2548 | 107 | Code::CodeKey { rung: Doc, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.697 |
| ns | 2586 |  | 294 | InboundApi: complete method roster with endpoints | 2.7 |  | 0.664 |
| walker |  | 2646 | 98 | Markdown::HeadingsOutline { file: py3xui/server/README.md } |  |  | 0.664 |
| ns | 2750 |  | 164 | ServerApi and DatabaseApi: complete rosters with endpoints | 2.8 |  | 0.642 |
| walker |  | 2789 | 143 | Markdown::HeadingsOutline { file: py3xui/api/README.md } |  |  | 0.642 |
| ns | 2926 |  | 176 | Api.from_env signature and the complete environment-variable contract | 2.9 |  | 0.621 |
| walker |  | 2940 | 151 | Markdown::HeadingsOutline { file: py3xui/inbound/README.md } |  |  | 0.621 |
| walker |  | 3118 | 178 | Code::CodeKey { rung: Body, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.621 |
| ns | 3158 |  | 232 | from_env body and Api.login: the two-step connect sequence | 2.10 | 2.9 | 0.594 |
| walker |  | 3281 | 163 | Markdown::HeadingsOutline { file: py3xui/async_api/README.md } |  |  | 0.594 |
| ns | 3400 |  | 242 | AsyncClientApi: complete async method roster | 2.11 |  | 0.578 |
| walker |  | 3435 | 154 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.578 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.559 |
| walker |  | 3672 | 237 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.584 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.575 |
| walker |  | 3869 | 197 | Markdown::Section { file: py3xui/api/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.569 |
| walker |  | 3992 | 123 | Plaintext::Whole { file: dev/requirements.txt } |  |  | 0.569 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.565 |
| walker |  | 4197 | 205 | Markdown::Section { file: py3xui/async_api/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.565 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.548 |
| walker |  | 4385 | 188 | Markdown::Section { file: py3xui/async_api/README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.548 |
| walker |  | 4448 | 63 | Markdown::Section { file: py3xui/server/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 4512 | 64 | Markdown::Section { file: py3xui/client/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.548 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.535 |
| walker |  | 4534 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 4612 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.536 |
| walker |  | 4628 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.536 |
| walker |  | 4639 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 4759 | 120 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.537 |
| walker |  | 4767 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.537 |
| walker |  | 4775 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.537 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.513 |
| walker |  | 4784 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.513 |
| walker |  | 4794 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 6, sub: 0, line: 124 } |  |  | 0.513 |
| walker |  | 4861 | 67 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.519 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.507 |
| walker |  | 4956 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.532 |
| walker |  | 4966 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.532 |
| walker |  | 5013 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.532 |
| walker |  | 5062 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.532 |
| walker |  | 5085 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.520 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.504 |
| walker |  | 5312 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.549 |
| walker |  | 5328 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.558 |
| walker |  | 5340 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.553 |
| walker |  | 5461 | 121 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.553 |
| walker |  | 5469 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.553 |
| walker |  | 5477 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.553 |
| walker |  | 5486 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.553 |
| walker |  | 5496 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 6, sub: 0, line: 129 } |  |  | 0.553 |
| walker |  | 5564 | 68 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.553 |
| walker |  | 5659 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.567 |
| walker |  | 5669 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.567 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.546 |
| walker |  | 5716 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.546 |
| walker |  | 5765 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.546 |
| walker |  | 5789 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5824 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.546 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.536 |
| walker |  | 5861 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.536 |
| walker |  | 5877 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.536 |
| walker |  | 5933 | 56 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.536 |
| walker |  | 6042 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 6060 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.536 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.526 |
| walker |  | 6078 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.526 |
| walker |  | 6096 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.527 |
| walker |  | 6116 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.527 |
| walker |  | 6143 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.527 |
| walker |  | 6186 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.528 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.520 |
| walker |  | 6271 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.521 |
| walker |  | 6291 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.521 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.513 |
| walker |  | 6343 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.513 |
| walker |  | 6397 | 54 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.513 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.505 |
| walker |  | 6737 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.507 |
| walker |  | 6794 | 57 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.507 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.527 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.545 |
| walker |  | 7143 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.545 |
| walker |  | 7157 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.545 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.551 |
| walker |  | 7182 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.554 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.561 |
| walker |  | 7363 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.583 |
| walker |  | 7379 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.588 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.578 |
| walker |  | 7720 | 341 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.616 |
| walker |  | 7756 | 36 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.618 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.607 |
| walker |  | 7812 | 56 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 4, sub: 0, line: 114 } |  |  | 0.614 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.605 |
| walker |  | 8169 | 357 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.627 |
| walker |  | 8177 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.627 |
| walker |  | 8185 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.627 |
| walker |  | 8193 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.627 |
| walker |  | 8201 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.627 |
| walker |  | 8209 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.627 |
| walker |  | 8217 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.627 |
| walker |  | 8225 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.627 |
| walker |  | 8233 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.627 |
| walker |  | 8241 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.627 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.617 |
| walker |  | 8250 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.617 |
| walker |  | 8260 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.617 |
| walker |  | 8271 | 11 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.617 |
| walker |  | 8309 | 38 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.617 |
| walker |  | 8369 | 60 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.617 |
| walker |  | 8464 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.621 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.611 |
| walker |  | 8505 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.611 |
| walker |  | 8546 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.611 |
| walker |  | 8587 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.611 |
| walker |  | 8630 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.611 |
| walker |  | 8673 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.611 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.604 |
| walker |  | 8716 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.604 |
| walker |  | 8761 | 45 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.604 |
| walker |  | 8807 | 46 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.604 |
| walker |  | 8855 | 48 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.604 |
| walker |  | 8904 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.604 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.593 |
| walker |  | 8956 | 52 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.593 |
| walker |  | 9013 | 57 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.593 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.589 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.591 |
| walker |  | 9254 | 241 | Markdown::Section { file: py3xui/inbound/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.583 |
| walker |  | 9496 | 242 | Markdown::Section { file: py3xui/api/README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.583 |
| walker |  | 9555 | 59 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.583 |
| walker |  | 9615 | 60 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.583 |
| walker |  | 9628 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.572 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.570 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.569 |
| walker |  | 9994 | 366 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.569 |

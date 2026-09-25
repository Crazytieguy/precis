Score(3000)=0.621 I=0.857 C=0.450 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.746/0.723/0.683/0.621/0.525/0.505/0.582

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
| walker |  | 3632 | 197 | Markdown::Section { file: py3xui/api/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.578 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.559 |
| walker |  | 3755 | 123 | Plaintext::Whole { file: dev/requirements.txt } |  |  | 0.560 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.550 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.545 |
| walker |  | 3960 | 205 | Markdown::Section { file: py3xui/async_api/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 4023 | 63 | Markdown::Section { file: py3xui/server/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 4087 | 64 | Markdown::Section { file: py3xui/client/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.545 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.540 |
| walker |  | 4109 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 4187 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.541 |
| walker |  | 4203 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.541 |
| walker |  | 4214 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.525 |
| walker |  | 4334 | 120 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.526 |
| walker |  | 4342 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.526 |
| walker |  | 4350 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.526 |
| walker |  | 4359 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.526 |
| walker |  | 4369 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 6, sub: 0, line: 124 } |  |  | 0.526 |
| walker |  | 4436 | 67 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.533 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.521 |
| walker |  | 4531 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.548 |
| walker |  | 4541 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.548 |
| walker |  | 4588 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.548 |
| walker |  | 4637 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.548 |
| walker |  | 4660 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.523 |
| walker |  | 4887 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.574 |
| walker |  | 4903 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.584 |
| walker |  | 4915 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.570 |
| walker |  | 5036 | 121 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.571 |
| walker |  | 5044 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.571 |
| walker |  | 5052 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.571 |
| walker |  | 5061 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.571 |
| walker |  | 5071 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 6, sub: 0, line: 129 } |  |  | 0.571 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.557 |
| walker |  | 5139 | 68 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.557 |
| walker |  | 5234 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.572 |
| walker |  | 5244 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.572 |
| walker |  | 5291 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.572 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.554 |
| walker |  | 5340 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.554 |
| walker |  | 5364 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.548 |
| walker |  | 5399 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.548 |
| walker |  | 5436 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.548 |
| walker |  | 5452 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.548 |
| walker |  | 5508 | 56 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.548 |
| walker |  | 5617 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5635 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.549 |
| walker |  | 5653 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.549 |
| walker |  | 5671 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.549 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.529 |
| walker |  | 5691 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.529 |
| walker |  | 5718 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.529 |
| walker |  | 5761 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.530 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.520 |
| walker |  | 5846 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.521 |
| walker |  | 5866 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.521 |
| walker |  | 5918 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.521 |
| walker |  | 5972 | 54 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.521 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.512 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.504 |
| walker |  | 6312 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.506 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.499 |
| walker |  | 6369 | 57 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.499 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.491 |
| walker |  | 6718 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.491 |
| walker |  | 6732 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.491 |
| walker |  | 6757 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.512 |
| walker |  | 6938 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.538 |
| walker |  | 6954 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.544 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.561 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.566 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.568 |
| walker |  | 7295 | 341 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.609 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.614 |
| walker |  | 7331 | 36 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.616 |
| walker |  | 7387 | 56 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 4, sub: 0, line: 114 } |  |  | 0.624 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.612 |
| walker |  | 7744 | 357 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.614 |
| walker |  | 7752 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.614 |
| walker |  | 7760 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.614 |
| walker |  | 7768 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.614 |
| walker |  | 7776 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.614 |
| walker |  | 7784 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.614 |
| walker |  | 7792 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.614 |
| walker |  | 7800 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.614 |
| walker |  | 7808 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.614 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.624 |
| walker |  | 7816 | 8 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.624 |
| walker |  | 7825 | 9 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.624 |
| walker |  | 7835 | 10 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.624 |
| walker |  | 7846 | 11 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.624 |
| walker |  | 7884 | 38 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.624 |
| walker |  | 7944 | 60 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.624 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.615 |
| walker |  | 8039 | 95 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.619 |
| walker |  | 8080 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 4, sub: 0, line: 80 } |  |  | 0.619 |
| walker |  | 8121 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 5, sub: 0, line: 88 } |  |  | 0.619 |
| walker |  | 8162 | 41 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 6, sub: 0, line: 96 } |  |  | 0.619 |
| walker |  | 8205 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 7, sub: 0, line: 104 } |  |  | 0.619 |
| walker |  | 8248 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 8, sub: 0, line: 112 } |  |  | 0.609 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.609 |
| walker |  | 8291 | 43 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 9, sub: 0, line: 120 } |  |  | 0.609 |
| walker |  | 8336 | 45 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 11, sub: 0, line: 136 } |  |  | 0.609 |
| walker |  | 8382 | 46 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 10, sub: 0, line: 128 } |  |  | 0.609 |
| walker |  | 8430 | 48 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 12, sub: 0, line: 144 } |  |  | 0.609 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.600 |
| walker |  | 8479 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 13, sub: 0, line: 152 } |  |  | 0.600 |
| walker |  | 8531 | 52 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 14, sub: 0, line: 160 } |  |  | 0.600 |
| walker |  | 8588 | 57 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 17, sub: 0, line: 210 } |  |  | 0.600 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.592 |
| walker |  | 8829 | 241 | Markdown::Section { file: py3xui/inbound/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.592 |
| walker |  | 8888 | 59 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.592 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.582 |
| walker |  | 8948 | 60 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.582 |
| walker |  | 8961 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.577 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.580 |
| walker |  | 9327 | 366 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.580 |
| walker |  | 9335 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.580 |
| walker |  | 9343 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.580 |
| walker |  | 9351 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.580 |
| walker |  | 9359 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.580 |
| walker |  | 9367 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.580 |
| walker |  | 9375 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.580 |
| walker |  | 9383 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.580 |
| walker |  | 9391 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 12, sub: 0, line: 140 } |  |  | 0.580 |
| walker |  | 9399 | 8 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 18, sub: 0, line: 281 } |  |  | 0.580 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.572 |
| walker |  | 9408 | 9 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.572 |
| walker |  | 9418 | 10 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 13, sub: 0, line: 148 } |  |  | 0.572 |
| walker |  | 9429 | 11 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.572 |
| walker |  | 9468 | 39 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 20, sub: 0, line: 308 } |  |  | 0.572 |
| walker |  | 9526 | 58 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 15, sub: 0, line: 166 } |  |  | 0.574 |
| walker |  | 9621 | 95 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 2, sub: 0, line: 49 } |  |  | 0.574 |
| walker |  | 9662 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 3, sub: 0, line: 68 } |  |  | 0.574 |
| walker |  | 9703 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 4, sub: 0, line: 76 } |  |  | 0.574 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.563 |
| walker |  | 9744 | 41 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 5, sub: 0, line: 84 } |  |  | 0.563 |
| walker |  | 9787 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 6, sub: 0, line: 92 } |  |  | 0.563 |
| walker |  | 9830 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 7, sub: 0, line: 100 } |  |  | 0.563 |
| walker |  | 9873 | 43 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 8, sub: 0, line: 108 } |  |  | 0.563 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.562 |
| walker |  | 9918 | 45 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 10, sub: 0, line: 124 } |  |  | 0.562 |
| walker |  | 9964 | 46 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 9, sub: 0, line: 116 } |  |  | 0.562 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.560 |
| walker |  | 9992 | 28 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api_base.py, decl: 11, sub: 0, line: 132 } |  |  | 0.560 |

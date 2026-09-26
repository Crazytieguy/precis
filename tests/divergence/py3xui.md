Score(3000)=0.614 I=0.871 C=0.432 ns_rows≤3K=20/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.862/0.802/0.690/0.614/0.582/0.672/0.712

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 74 |  | 74 | What py3xui is: distribution name, version, one-line purpose | 1.1 |  | 0.000 |
| walker |  | 81 | 25 | Fs::DirListing { dir: py3xui } |  |  | 0.000 |
| walker |  | 95 | 14 | Fs::DirListing { dir: py3xui/client } |  |  | 0.000 |
| walker |  | 109 | 14 | Fs::DirListing { dir: py3xui/server } |  |  | 0.000 |
| walker |  | 123 | 14 | Fs::DirListing { dir: py3xui/utils } |  |  | 0.000 |
| ns | 130 |  | 56 | Complete repository root listing | 1.2 |  | 0.628 |
| ns | 212 |  | 82 | Top-level public exports: py3xui/__init__.py in full | 1.3 |  | 0.542 |
| ns | 237 |  | 25 | py3xui/ package listing — the six sub-packages | 1.4 |  | 0.559 |
| walker |  | 311 | 188 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.571 |
| walker |  | 338 | 27 | Fs::DirListing { dir: dev } |  |  | 0.571 |
| walker |  | 370 | 32 | Fs::DirListing { dir: py3xui/inbound } |  |  | 0.591 |
| ns | 377 |  | 140 | README Overview: purpose and runtime dependencies | 1.5 |  | 0.558 |
| walker |  | 410 | 40 | Fs::DirListing { dir: py3xui/api } |  |  | 0.567 |
| walker |  | 481 | 71 | Toml::Identity { file: pyproject.toml } |  |  | 0.682 |
| walker |  | 527 | 46 | Fs::DirListing { dir: py3xui/async_api } |  |  | 0.703 |
| walker |  | 586 | 59 | Markdown::Prelude { file: README.md } |  |  | 0.877 |
| ns | 590 |  | 213 | Every remaining README section heading (complete map of the root README) | 1.6 |  | 0.704 |
| walker |  | 598 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.704 |
| walker |  | 616 | 18 | Fs::DirListing { dir: .github } |  |  | 0.704 |
| walker |  | 639 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.705 |
| walker |  | 698 | 59 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.705 |
| ns | 709 |  | 119 | Supported Python versions and 3x-ui compatibility floor | 1.7 |  | 0.661 |
| walker |  | 725 | 27 | Fs::DirListing { dir: tests } |  |  | 0.662 |
| walker |  | 739 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.662 |
| ns | 795 |  | 86 | Listings of the two transport packages: py3xui/api/ and py3xui/async_api/ | 1.8 |  | 0.718 |
| ns | 869 |  | 74 | Listings of the model and utility packages: inbound/, client/, server/, utils/ | 1.9 |  | 0.746 |
| walker |  | 906 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.830 |
| ns | 1016 |  | 147 | Re-export blocks of py3xui/api/__init__.py and py3xui/async_api/__init__.py | 1.10 |  | 0.789 |
| walker |  | 1068 | 162 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.859 |
| walker |  | 1111 | 43 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.859 |
| ns | 1157 |  | 141 | Re-export blocks of the inbound/, client/, server/ and utils/ packages | 1.11 |  | 0.816 |
| walker |  | 1193 | 82 | Code::CodeKey { rung: Names, file: py3xui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.861 |
| ns | 1342 |  | 185 | Canonical usage: env-var credentials, Api.from_env() and AsyncApi.from_env() | 2.1 |  | 0.802 |
| ns | 1459 |  | 117 | class Api and its full constructor signature | 2.2 |  | 0.762 |
| walker |  | 1541 | 348 | Code::CodeKey { rung: Names, file: demo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 1665 |  | 206 | Api constructor body: the four sub-API attributes it wires up | 2.3 | 2.2 | 0.713 |
| walker |  | 1695 | 154 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.713 |
| walker |  | 1706 | 11 | Code::CodeKey { rung: Names, file: py3xui/api/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| ns | 1840 |  | 175 | class AsyncApi: identical constructor, Async* sub-APIs | 2.4 |  | 0.674 |
| walker |  | 1902 | 196 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 1, sub: 0, line: 13 } |  |  | 0.678 |
| walker |  | 1950 | 48 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 7, sub: 0, line: 139 } |  |  | 0.678 |
| walker |  | 2023 | 73 | Code::CodeKey { rung: Decl, file: py3xui/api/api.py, decl: 2, sub: 0, line: 67 } |  |  | 0.719 |
| walker |  | 2046 | 23 | Code::CodeKey { rung: Names, file: py3xui/client/client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| ns | 2063 |  | 223 | ClientApi methods 1-5 with the panel endpoint each one calls | 2.5 |  | 0.690 |
| walker |  | 2273 | 227 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.694 |
| ns | 2292 |  | 229 | ClientApi methods 6-10 with their endpoints (closes the ClientApi roster) | 2.6 | 2.5 | 0.672 |
| ns | 2586 |  | 294 | InboundApi: complete method roster with endpoints | 2.7 |  | 0.640 |
| walker |  | 2683 | 410 | Code::CodeKey { rung: Decl, file: py3xui/client/client.py, decl: 2, sub: 0, line: 36 } |  |  | 0.646 |
| walker |  | 2695 | 12 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| ns | 2750 |  | 164 | ServerApi and DatabaseApi: complete rosters with endpoints | 2.8 |  | 0.625 |
| walker |  | 2893 | 198 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 1, sub: 0, line: 18 } |  |  | 0.627 |
| ns | 2926 |  | 176 | Api.from_env signature and the complete environment-variable contract | 2.9 |  | 0.614 |
| walker |  | 2941 | 48 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 7, sub: 0, line: 144 } |  |  | 0.614 |
| walker |  | 3014 | 73 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api.py, decl: 2, sub: 0, line: 72 } |  |  | 0.634 |
| walker |  | 3123 | 109 | Code::CodeKey { rung: Names, file: py3xui/server/server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 3141 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 2, sub: 0, line: 43 } |  |  | 0.634 |
| ns | 3158 |  | 232 | from_env body and Api.login: the two-step connect sequence | 2.10 | 2.9 | 0.607 |
| walker |  | 3159 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 4, sub: 0, line: 67 } |  |  | 0.607 |
| walker |  | 3177 | 18 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.607 |
| walker |  | 3197 | 20 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 6, sub: 0, line: 89 } |  |  | 0.608 |
| walker |  | 3224 | 27 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 7, sub: 0, line: 100 } |  |  | 0.608 |
| walker |  | 3267 | 43 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 3, sub: 0, line: 54 } |  |  | 0.609 |
| walker |  | 3352 | 85 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.609 |
| ns | 3400 |  | 242 | AsyncClientApi: complete async method roster | 2.11 |  | 0.593 |
| ns | 3667 |  | 267 | AsyncInboundApi, AsyncServerApi, AsyncDatabaseApi: complete async rosters | 2.12 | 2.11 | 0.574 |
| walker |  | 3701 | 349 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.574 |
| ns | 3789 |  | 122 | py3xui/utils/env.py: every public function, signatures only | 2.13 |  | 0.564 |
| ns | 3954 |  | 165 | TLS configuration: disabling verification vs. supplying a custom certificate | 2.14 |  | 0.558 |
| walker |  | 4041 | 340 | Code::CodeKey { rung: Decl, file: py3xui/server/server.py, decl: 9, sub: 0, line: 125 } |  |  | 0.560 |
| walker |  | 4066 | 25 | Code::CodeKey { rung: Names, file: py3xui/inbound/inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| ns | 4102 |  | 148 | Two-factor login and the URI-path gotcha | 2.15 |  | 0.556 |
| walker |  | 4247 | 181 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.558 |
| ns | 4301 |  | 199 | class Client: required fields and the first block of optional ones | 3.1 |  | 0.581 |
| ns | 4520 |  | 219 | class Client: remaining fields and model_config | 3.2 | 3.1 | 0.596 |
| walker |  | 4617 | 370 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 2, sub: 0, line: 38 } |  |  | 0.600 |
| walker |  | 4629 | 12 | Code::CodeKey { rung: Decl, file: py3xui/inbound/inbound.py, decl: 3, sub: 0, line: 88 } |  |  | 0.600 |
| walker |  | 4643 | 14 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.601 |
| walker |  | 4669 | 26 | Code::CodeKey { rung: Names, file: py3xui/inbound/stream_settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| ns | 4778 |  | 258 | ClientFields: the complete python-name to panel-JSON-key mapping | 3.3 |  | 0.618 |
| walker |  | 4792 | 123 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.619 |
| ns | 4921 |  | 143 | class Inbound: required fields and the first block of optionals | 3.4 |  | 0.632 |
| ns | 5088 |  | 167 | class Inbound: traffic counters, expiry, client_stats and model_config | 3.5 | 3.4 | 0.644 |
| walker |  | 5124 | 332 | Code::CodeKey { rung: Decl, file: py3xui/inbound/stream_settings.py, decl: 2, sub: 0, line: 25 } |  |  | 0.646 |
| walker |  | 5152 | 28 | Code::CodeKey { rung: Names, file: py3xui/inbound/sniffing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 5211 | 59 | Code::CodeKey { rung: Decl, file: py3xui/inbound/sniffing.py, decl: 1, sub: 0, line: 9 } |  |  | 0.646 |
| ns | 5301 |  | 213 | InboundFields: the complete inbound JSON key mapping | 3.6 |  | 0.653 |
| walker |  | 5322 | 111 | Code::CodeKey { rung: Decl, file: py3xui/inbound/sniffing.py, decl: 2, sub: 0, line: 20 } |  |  | 0.653 |
| walker |  | 5346 | 24 | Code::CodeKey { rung: Names, file: py3xui/inbound/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| ns | 5375 |  | 74 | Inbound.to_json signature and contract | 3.7 |  | 0.648 |
| walker |  | 5381 | 35 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 2, sub: 0, line: 17 } |  |  | 0.648 |
| walker |  | 5418 | 37 | Code::CodeKey { rung: Decl, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.648 |
| walker |  | 5434 | 16 | Code::CodeKey { rung: Doc, file: py3xui/client/client.py, decl: 1, sub: 0, line: 7 } |  |  | 0.656 |
| walker |  | 5450 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/inbound.py, decl: 1, sub: 0, line: 15 } |  |  | 0.663 |
| walker |  | 5466 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.663 |
| walker |  | 5482 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/sniffing.py, decl: 1, sub: 0, line: 9 } |  |  | 0.663 |
| walker |  | 5498 | 16 | Code::CodeKey { rung: Doc, file: py3xui/inbound/stream_settings.py, decl: 1, sub: 0, line: 9 } |  |  | 0.663 |
| ns | 5681 |  | 306 | Inbound.to_json body: which fields are sent, and the nested-JSON-string encoding | 3.8 | 3.7 | 0.639 |
| walker |  | 5735 | 237 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.655 |
| walker |  | 5755 | 20 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 8, sub: 0, line: 113 } |  |  | 0.655 |
| walker |  | 5765 | 10 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.655 |
| walker |  | 5775 | 10 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.655 |
| ns | 5829 |  | 148 | Inbound.validate_stream_settings: the dict / JSON-string / empty-string union | 3.9 |  | 0.646 |
| walker |  | 5870 | 95 | Code::CodeKey { rung: Names, file: py3xui/utils/env.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 5897 | 27 | Code::CodeKey { rung: Decl, file: py3xui/utils/env.py, decl: 1, sub: 0, line: 7 } |  |  | 0.659 |
| walker |  | 5908 | 11 | Code::CodeKey { rung: Body, file: py3xui/api/api.py, decl: 5, sub: 0, line: 115 } |  |  | 0.659 |
| walker |  | 5919 | 11 | Code::CodeKey { rung: Body, file: py3xui/async_api/async_api.py, decl: 5, sub: 0, line: 120 } |  |  | 0.659 |
| walker |  | 5935 | 16 | Code::CodeKey { rung: Names, file: py3xui/client/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 6016 | 81 | Plaintext::DeclSurface { file: dev/push.sh } |  |  | 0.659 |
| ns | 6066 |  | 237 | Settings and Sniffing models with their field-name constants | 3.10 |  | 0.667 |
| walker |  | 6098 | 82 | Plaintext::Whole { file: dev/push.sh } |  |  | 0.667 |
| walker |  | 6181 | 83 | Plaintext::DeclSurface { file: dev/requirements.txt } |  |  | 0.667 |
| ns | 6205 |  | 139 | StreamSettingsFields: every transport-settings JSON key | 3.11 |  | 0.672 |
| walker |  | 6221 | 40 | Plaintext::Whole { file: dev/requirements.txt } |  |  | 0.672 |
| walker |  | 6289 | 68 | Code::CodeKey { rung: Names, file: py3xui/api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 6311 | 22 | Code::CodeKey { rung: Names, file: py3xui/api/api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 6335 |  | 130 | class StreamSettings: every field, protocol dicts truncated to their names | 3.12 | 3.11 | 0.682 |
| walker |  | 6389 | 78 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.682 |
| walker |  | 6403 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| ns | 6478 |  | 143 | JsonStringModel: the base class that parses JSON-string fields | 3.13 |  | 0.671 |
| walker |  | 6618 | 215 | Code::CodeKey { rung: Decl, file: py3xui/api/api_client.py, decl: 1, sub: 0, line: 13 } |  |  | 0.680 |
| walker |  | 6633 | 15 | Code::CodeKey { rung: Names, file: py3xui/api/api_inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 6770 | 137 | Code::CodeKey { rung: Decl, file: py3xui/api/api_inbound.py, decl: 1, sub: 0, line: 9 } |  |  | 0.685 |
| walker |  | 6784 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| ns | 6831 |  | 353 | class Server: complete field declarations for the status payload | 3.14 |  | 0.695 |
| walker |  | 6838 | 54 | Code::CodeKey { rung: Decl, file: py3xui/api/api_server.py, decl: 1, sub: 0, line: 7 } |  |  | 0.697 |
| walker |  | 6852 | 14 | Code::CodeKey { rung: Names, file: py3xui/api/api_database.py, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 6866 | 14 | Code::CodeKey { rung: Decl, file: py3xui/api/api_database.py, decl: 1, sub: 0, line: 7 } |  |  | 0.699 |
| ns | 7050 |  | 219 | The six nested server sub-models, class lines plus fields | 3.15 | 3.14 | 0.706 |
| walker |  | 7094 | 228 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 0, line: 28 } |  |  | 0.706 |
| ns | 7161 |  | 111 | class RealityKeyPair | 3.16 |  | 0.709 |
| walker |  | 7167 | 73 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 3, sub: 0, line: 61 } |  |  | 0.709 |
| ns | 7187 |  | 26 | Location of ServerFields, the server-status alias table | 3.17 | 3.14 | 0.710 |
| ns | 7303 |  | 116 | ApiFields response-envelope constants and class BaseApi | 4.1 |  | 0.709 |
| walker |  | 7444 | 277 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 2, sub: 1, line: 28 } |  |  | 0.711 |
| walker |  | 7470 | 26 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 21, sub: 0, line: 313 } |  |  | 0.711 |
| walker |  | 7518 | 48 | Code::CodeKey { rung: Decl, file: py3xui/api/api_base.py, decl: 20, sub: 0, line: 246 } |  |  | 0.711 |
| walker |  | 7534 | 16 | Code::CodeKey { rung: Doc, file: py3xui/api/api_base.py, decl: 1, sub: 0, line: 15 } |  |  | 0.715 |
| ns | 7536 |  | 233 | BaseApi.__init__: the private state every sub-API carries | 4.2 | 4.1 | 0.706 |
| walker |  | 7604 | 70 | Code::CodeKey { rung: Names, file: py3xui/inbound/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 7619 | 15 | Code::CodeKey { rung: Names, file: py3xui/inbound/bases.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 7664 | 45 | Code::CodeKey { rung: Decl, file: py3xui/inbound/bases.py, decl: 1, sub: 0, line: 9 } |  |  | 0.713 |
| walker |  | 7676 | 12 | Code::CodeKey { rung: Decl, file: py3xui/inbound/bases.py, decl: 2, sub: 0, line: 12 } |  |  | 0.714 |
| walker |  | 7693 | 17 | Code::CodeKey { rung: Doc, file: py3xui/inbound/bases.py, decl: 1, sub: 0, line: 9 } |  |  | 0.716 |
| walker |  | 7728 | 35 | Code::CodeKey { rung: Names, file: py3xui/utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 7807 | 79 | Code::CodeKey { rung: Names, file: py3xui/async_api/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| ns | 7809 |  | 273 | BaseApi: every member, signatures only | 4.3 | 4.1 | 0.737 |
| walker |  | 7820 | 13 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 7837 | 17 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| ns | 8024 |  | 215 | How the TLS `verify` argument is chosen, per request | 4.4 | 4.3 | 0.726 |
| walker |  | 8062 | 225 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_client.py, decl: 1, sub: 0, line: 12 } |  |  | 0.739 |
| walker |  | 8079 | 17 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_inbound.py, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 8223 | 144 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_inbound.py, decl: 1, sub: 0, line: 11 } |  |  | 0.745 |
| walker |  | 8240 | 17 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| ns | 8248 |  | 224 | The retry loop: which errors retry, the backoff, and what is raised at the end | 4.5 | 4.4 | 0.734 |
| walker |  | 8297 | 57 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_server.py, decl: 1, sub: 0, line: 8 } |  |  | 0.739 |
| walker |  | 8314 | 17 | Code::CodeKey { rung: Names, file: py3xui/async_api/async_api_database.py, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 8329 | 15 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_database.py, decl: 1, sub: 0, line: 7 } |  |  | 0.743 |
| ns | 8467 |  | 219 | BaseApi.login: the POST that mints the session cookie | 4.6 | 4.3 | 0.732 |
| walker |  | 8557 | 228 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 0, line: 16 } |  |  | 0.732 |
| walker |  | 8630 | 73 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 2, sub: 0, line: 49 } |  |  | 0.732 |
| ns | 8675 |  | 208 | Cookie discovery, cookie dict, and the login-required guards | 4.7 | 4.6 | 0.723 |
| walker |  | 8918 | 288 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 1, sub: 1, line: 16 } |  |  | 0.723 |
| ns | 8924 |  | 249 | AsyncBaseApi: where the async transport actually differs | 4.8 | 4.5 | 0.711 |
| walker |  | 8944 | 26 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 20, sub: 0, line: 308 } |  |  | 0.711 |
| walker |  | 8989 | 45 | Code::CodeKey { rung: Decl, file: py3xui/async_api/async_api_base.py, decl: 15, sub: 0, line: 166 } |  |  | 0.712 |
| walker |  | 9009 | 20 | Code::CodeKey { rung: Names, file: py3xui/server/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| ns | 9075 |  | 151 | Async error handling: different exception types, different terminal error | 4.9 | 4.8 | 0.710 |
| walker |  | 9116 | 107 | Code::CodeKey { rung: Doc, file: demo.py, decl: 14, sub: 0, line: 70 } |  |  | 0.710 |
| walker |  | 9163 | 47 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 3, sub: 0, line: 93 } |  |  | 0.710 |
| walker |  | 9210 | 47 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 3, sub: 0, line: 98 } |  |  | 0.710 |
| ns | 9230 |  | 155 | Every remaining directory in the repository, listed in full | 5.1 |  | 0.710 |
| walker |  | 9342 | 132 | Plaintext::DeclSurface { file: dev/pydoc.sh } |  |  | 0.710 |
| walker |  | 9391 | 49 | Code::CodeKey { rung: Doc, file: py3xui/api/api.py, decl: 4, sub: 0, line: 102 } |  |  | 0.710 |
| ns | 9403 |  | 173 | tests/test_api.py: how the suite is wired | 5.2 |  | 0.701 |
| walker |  | 9440 | 49 | Code::CodeKey { rung: Doc, file: py3xui/async_api/async_api.py, decl: 4, sub: 0, line: 107 } |  |  | 0.701 |
| walker |  | 9575 | 135 | Plaintext::DeclSurface { file: dev/clean_trash.sh } |  |  | 0.701 |
| walker |  | 9685 | 110 | Plaintext::Whole { file: dev/clean_trash.sh } |  |  | 0.701 |
| ns | 9719 |  | 316 | tests/test_api.py: every test function in the file | 5.3 | 5.2 | 0.687 |
| walker |  | 9737 | 52 | Code::CodeKey { rung: Doc, file: py3xui/server/server.py, decl: 5, sub: 0, line: 78 } |  |  | 0.687 |
| ns | 9910 |  | 191 | Lint configuration and the development dependency set | 5.4 |  | 0.683 |
| ns | 9981 |  | 71 | The per-package README.md files are generated, not written | 5.5 |  | 0.681 |
| walker |  | 9997 | 260 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.690 |

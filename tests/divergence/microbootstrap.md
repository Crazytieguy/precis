Score(3000)=0.778 I=0.928 C=0.653 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.589/0.804/0.710/0.778/0.691/0.677/0.638

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 30 |  | 30 | README tagline: what microbootstrap is, in one sentence | 1.1 |  | 0.000 |
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 36 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 45 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 86 | 41 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 126 |  | 96 | The complete list of built-in instruments | 1.2 |  | 0.246 |
| walker |  | 158 | 72 | Toml::Identity { file: pyproject.toml } |  |  | 0.246 |
| ns | 191 |  | 65 | The four bootstrap targets: fastapi, litestar, faststream, or no framework | 1.3 |  | 0.176 |
| ns | 224 |  | 33 | Repository root listing (complete) | 1.4 |  | 0.429 |
| walker |  | 230 | 72 | Json::Identity { file: package.json } |  |  | 0.429 |
| walker |  | 287 | 57 | Fs::DirListing { dir: microbootstrap } |  |  | 0.456 |
| walker |  | 305 | 18 | Fs::DirListing { dir: microbootstrap/middlewares } |  |  | 0.457 |
| walker |  | 328 | 23 | Fs::DirListing { dir: microbootstrap/config } |  |  | 0.463 |
| ns | 353 |  | 129 | `microbootstrap/` and `microbootstrap/instruments/` listings (complete) | 1.5 |  | 0.386 |
| walker |  | 355 | 27 | Fs::DirListing { dir: microbootstrap/bootstrappers } |  |  | 0.398 |
| walker |  | 427 | 72 | Fs::DirListing { dir: microbootstrap/instruments } |  |  | 0.610 |
| ns | 439 |  | 86 | `bootstrappers/`, `config/`, `middlewares/`, `examples/` listings (complete) | 1.6 |  | 0.586 |
| walker |  | 467 | 40 | Json::Dependencies { file: package.json } |  |  | 0.586 |
| walker |  | 485 | 18 | Fs::DirListing { dir: examples } |  |  | 0.632 |
| walker |  | 568 | 83 | Json::Scripts { file: package.json } |  |  | 0.632 |
| walker |  | 615 | 47 | Fs::DirListing { dir: tests } |  |  | 0.633 |
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.541 |
| walker |  | 808 | 193 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.473 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.415 |
| walker |  | 1183 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.773 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.743 |
| walker |  | 1477 | 294 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.843 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.773 |
| walker |  | 1704 | 227 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.847 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.757 |
| walker |  | 1915 | 211 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.757 |
| walker |  | 1953 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.757 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.709 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.679 |
| walker |  | 2262 | 309 | Plaintext::Whole { file: Justfile } |  |  | 0.680 |
| walker |  | 2425 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 2476 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.721 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.685 |
| walker |  | 2540 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.691 |
| walker |  | 2635 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.716 |
| walker |  | 2730 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.757 |
| walker |  | 2828 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.788 |
| walker |  | 2838 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.794 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.754 |
| walker |  | 2849 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.760 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.738 |
| walker |  | 3100 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.792 |
| walker |  | 3112 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.799 |
| walker |  | 3124 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.805 |
| walker |  | 3136 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.773 |
| walker |  | 3245 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.773 |
| walker |  | 3295 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.773 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.756 |
| walker |  | 3359 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.757 |
| walker |  | 3372 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 3535 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.757 |
| walker |  | 3567 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.757 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.725 |
| walker |  | 3628 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.725 |
| walker |  | 3634 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.725 |
| walker |  | 3643 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.725 |
| walker |  | 3672 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| walker |  | 3724 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.725 |
| walker |  | 3803 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.725 |
| walker |  | 3848 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.709 |
| walker |  | 3857 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.709 |
| walker |  | 3873 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.709 |
| walker |  | 3890 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.709 |
| walker |  | 3920 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3983 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.710 |
| walker |  | 3991 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.710 |
| walker |  | 4118 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.711 |
| walker |  | 4125 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.711 |
| walker |  | 4155 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 4217 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.713 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.689 |
| walker |  | 4225 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.689 |
| walker |  | 4339 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.691 |
| walker |  | 4346 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.691 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.672 |
| walker |  | 4559 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.672 |
| walker |  | 4592 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| walker |  | 4684 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.676 |
| walker |  | 4692 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.676 |
| walker |  | 4843 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.676 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.656 |
| walker |  | 4850 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.656 |
| walker |  | 4858 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.656 |
| walker |  | 5020 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 5044 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.657 |
| walker |  | 5083 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.657 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.668 |
| walker |  | 5123 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.668 |
| walker |  | 5165 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.668 |
| walker |  | 5181 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.669 |
| walker |  | 5234 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| walker |  | 5266 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.671 |
| walker |  | 5352 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.674 |
| walker |  | 5360 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.674 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.667 |
| walker |  | 5478 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.688 |
| walker |  | 5486 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.688 |
| walker |  | 5497 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.688 |
| walker |  | 5635 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 5659 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.694 |
| walker |  | 5689 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.694 |
| walker |  | 5737 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.699 |
| walker |  | 5785 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.704 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.685 |
| walker |  | 5852 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.689 |
| walker |  | 5860 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.689 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.684 |
| walker |  | 5930 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.684 |
| walker |  | 6032 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.697 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.677 |
| walker |  | 6141 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.677 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.660 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.643 |
| walker |  | 6837 | 696 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.644 |
| walker |  | 6849 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/instrument_box.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.628 |
| walker |  | 6974 | 125 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 1, sub: 0, line: 9 } |  |  | 0.635 |
| walker |  | 6982 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.636 |
| walker |  | 7010 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 3, sub: 0, line: 21 } |  |  | 0.637 |
| walker |  | 7048 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.638 |
| walker |  | 7058 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.638 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.621 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.613 |
| walker |  | 7346 | 288 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 7370 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.622 |
| walker |  | 7400 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.625 |
| walker |  | 7438 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.626 |
| walker |  | 7446 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 17, sub: 0, line: 181 } |  |  | 0.626 |
| walker |  | 7486 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.627 |
| walker |  | 7551 | 65 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.627 |
| walker |  | 7621 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.627 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.609 |
| walker |  | 7713 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.610 |
| walker |  | 7806 | 93 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 10, sub: 0, line: 99 } |  |  | 0.617 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.604 |
| walker |  | 8164 | 358 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 4, sub: 0, line: 48 } |  |  | 0.629 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.618 |
| walker |  | 8399 | 235 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/logging_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 8417 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 8, sub: 0, line: 92 } |  |  | 0.628 |
| walker |  | 8456 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 9, sub: 0, line: 97 } |  |  | 0.629 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.625 |
| walker |  | 8519 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 4, sub: 0, line: 36 } |  |  | 0.626 |
| walker |  | 8625 | 106 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 10, sub: 0, line: 98 } |  |  | 0.626 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.621 |
| walker |  | 8743 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 6, sub: 0, line: 76 } |  |  | 0.621 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.619 |
| walker |  | 8885 | 142 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 14, sub: 0, line: 148 } |  |  | 0.630 |
| walker |  | 8893 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 21, sub: 0, line: 212 } |  |  | 0.630 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.635 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.635 |
| walker |  | 9090 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 12, sub: 0, line: 127 } |  |  | 0.645 |
| walker |  | 9103 | 13 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 13, sub: 0, line: 140 } |  |  | 0.646 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.638 |
| walker |  | 9308 | 205 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.638 |
| walker |  | 9321 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9334 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.634 |
| walker |  | 9531 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.634 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.643 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.650 |
| walker |  | 9785 | 254 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.650 |
| walker |  | 9806 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.651 |
| walker |  | 9821 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9913 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.658 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.664 |
| walker |  | 9974 | 61 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/sentry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |

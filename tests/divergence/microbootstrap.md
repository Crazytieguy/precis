Score(3000)=0.552 I=0.834 C=0.366 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.473/0.477/0.637/0.552/0.639/0.589/0.608

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 30 |  | 30 | README tagline: what microbootstrap is, in one sentence | 1.1 |  | 0.000 |
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 36 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 45 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 117 | 72 | Toml::Identity { file: pyproject.toml } |  |  | 0.000 |
| ns | 126 |  | 96 | The complete list of built-in instruments | 1.2 |  | 0.000 |
| walker |  | 174 | 57 | Fs::DirListing { dir: microbootstrap } |  |  | 0.000 |
| ns | 191 |  | 65 | The four bootstrap targets: fastapi, litestar, faststream, or no framework | 1.3 |  | 0.000 |
| walker |  | 192 | 18 | Fs::DirListing { dir: microbootstrap/middlewares } |  |  | 0.000 |
| walker |  | 215 | 23 | Fs::DirListing { dir: microbootstrap/config } |  |  | 0.000 |
| ns | 224 |  | 33 | Repository root listing (complete) | 1.4 |  | 0.294 |
| walker |  | 242 | 27 | Fs::DirListing { dir: microbootstrap/bootstrappers } |  |  | 0.315 |
| walker |  | 314 | 72 | Fs::DirListing { dir: microbootstrap/instruments } |  |  | 0.383 |
| walker |  | 332 | 18 | Fs::DirListing { dir: examples } |  |  | 0.396 |
| ns | 353 |  | 129 | `microbootstrap/` and `microbootstrap/instruments/` listings (complete) | 1.5 |  | 0.470 |
| walker |  | 403 | 71 | Plaintext::DeclSurface { file: Justfile } |  |  | 0.471 |
| ns | 439 |  | 86 | `bootstrappers/`, `config/`, `middlewares/`, `examples/` listings (complete) | 1.6 |  | 0.480 |
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.411 |
| walker |  | 662 | 259 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.541 |
| walker |  | 709 | 47 | Fs::DirListing { dir: tests } |  |  | 0.541 |
| walker |  | 808 | 99 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.541 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.473 |
| walker |  | 1033 | 225 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.487 |
| walker |  | 1071 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.488 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.496 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.477 |
| walker |  | 1446 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.828 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.759 |
| walker |  | 1510 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.761 |
| walker |  | 1522 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.761 |
| walker |  | 1631 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.761 |
| walker |  | 1681 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.761 |
| walker |  | 1727 | 46 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 4, sub: 0, line: 31 } |  |  | 0.761 |
| walker |  | 1779 | 52 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.761 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.680 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.637 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.610 |
| walker |  | 2473 | 694 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.611 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.581 |
| walker |  | 2486 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 2649 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.581 |
| walker |  | 2681 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.582 |
| walker |  | 2687 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.582 |
| walker |  | 2696 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.582 |
| walker |  | 2757 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.582 |
| walker |  | 2778 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.582 |
| walker |  | 2807 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.552 |
| walker |  | 2859 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.552 |
| walker |  | 2938 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.552 |
| walker |  | 2983 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 2992 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.552 |
| walker |  | 3008 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.553 |
| walker |  | 3025 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.553 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.536 |
| walker |  | 3188 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.543 |
| walker |  | 3239 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.551 |
| walker |  | 3249 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.554 |
| walker |  | 3313 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.559 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.546 |
| walker |  | 3408 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.568 |
| walker |  | 3420 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.572 |
| walker |  | 3515 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.611 |
| walker |  | 3527 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.617 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.591 |
| walker |  | 3625 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.620 |
| walker |  | 3636 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.626 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.612 |
| walker |  | 3887 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.661 |
| walker |  | 3918 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.661 |
| walker |  | 4145 | 227 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.639 |
| walker |  | 4307 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4331 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.639 |
| walker |  | 4370 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.639 |
| walker |  | 4410 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.639 |
| walker |  | 4452 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.640 |
| walker |  | 4468 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.640 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.622 |
| walker |  | 4532 | 64 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 2, sub: 0, line: 16 } |  |  | 0.622 |
| walker |  | 4792 | 260 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.668 |
| walker |  | 4804 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/instrument_box.py, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.649 |
| walker |  | 4929 | 125 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 1, sub: 0, line: 9 } |  |  | 0.659 |
| walker |  | 4937 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.660 |
| walker |  | 4965 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 3, sub: 0, line: 21 } |  |  | 0.661 |
| walker |  | 5003 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.661 |
| walker |  | 5013 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.662 |
| walker |  | 5033 | 20 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.664 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.649 |
| walker |  | 5148 | 115 | Code::CodeKey { rung: Body, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.649 |
| walker |  | 5161 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 5174 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 5371 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.649 |
| walker |  | 5402 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 7, sub: 0, line: 57 } |  |  | 0.649 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.629 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.611 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.606 |
| walker |  | 5949 | 547 | Toml::Config { file: pyproject.toml } |  |  | 0.606 |
| walker |  | 5964 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 6056 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.606 |
| walker |  | 6086 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.589 |
| walker |  | 6149 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.589 |
| walker |  | 6157 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.589 |
| walker |  | 6164 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.589 |
| walker |  | 6291 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.596 |
| walker |  | 6328 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 3, sub: 0, line: 22 } |  |  | 0.597 |
| walker |  | 6358 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.584 |
| walker |  | 6420 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.585 |
| walker |  | 6428 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.585 |
| walker |  | 6435 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.585 |
| walker |  | 6549 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.602 |
| walker |  | 6570 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.603 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.587 |
| walker |  | 6783 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.587 |
| walker |  | 6799 | 16 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 6825 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.587 |
| walker |  | 6858 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.573 |
| walker |  | 6950 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.576 |
| walker |  | 6958 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.576 |
| walker |  | 6965 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.576 |
| walker |  | 6973 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.576 |
| walker |  | 6989 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 3, sub: 0, line: 31 } |  |  | 0.577 |
| walker |  | 7140 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.584 |
| walker |  | 7157 | 17 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 7182 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.584 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.568 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.561 |
| walker |  | 7373 | 191 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 7396 | 23 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.562 |
| walker |  | 7421 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 3, sub: 0, line: 62 } |  |  | 0.562 |
| walker |  | 7447 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 11, sub: 0, line: 158 } |  |  | 0.562 |
| walker |  | 7477 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 5, sub: 0, line: 73 } |  |  | 0.562 |
| walker |  | 7507 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 7, sub: 0, line: 108 } |  |  | 0.562 |
| walker |  | 7537 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 13, sub: 0, line: 177 } |  |  | 0.562 |
| walker |  | 7567 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 15, sub: 0, line: 190 } |  |  | 0.562 |
| walker |  | 7619 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 17, sub: 0, line: 199 } |  |  | 0.562 |
| walker |  | 7627 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 19, sub: 0, line: 217 } |  |  | 0.562 |
| walker |  | 7679 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 20, sub: 0, line: 222 } |  |  | 0.563 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.571 |
| walker |  | 7760 | 81 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 1, sub: 0, line: 48 } |  |  | 0.581 |
| walker |  | 7813 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 7845 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.588 |
| walker |  | 7931 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.592 |
| walker |  | 7939 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.592 |
| walker |  | 7947 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.592 |
| walker |  | 7958 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.593 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.580 |
| walker |  | 8076 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.596 |
| walker |  | 8163 | 87 | Code::CodeKey { rung: Doc, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.596 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.586 |
| walker |  | 8314 | 151 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 8346 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 9, sub: 0, line: 73 } |  |  | 0.590 |
| walker |  | 8378 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 11, sub: 0, line: 92 } |  |  | 0.592 |
| walker |  | 8410 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 13, sub: 0, line: 103 } |  |  | 0.593 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.592 |
| walker |  | 8461 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 6, sub: 0, line: 57 } |  |  | 0.594 |
| walker |  | 8515 | 54 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 15, sub: 0, line: 113 } |  |  | 0.596 |
| walker |  | 8523 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.596 |
| walker |  | 8578 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 18, sub: 0, line: 136 } |  |  | 0.598 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.593 |
| walker |  | 8709 | 131 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 2, sub: 0, line: 27 } |  |  | 0.600 |
| walker |  | 8719 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 3, sub: 0, line: 33 } |  |  | 0.601 |
| walker |  | 8731 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 4, sub: 0, line: 41 } |  |  | 0.601 |
| walker |  | 8741 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.601 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.598 |
| walker |  | 8822 | 81 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.598 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.604 |
| walker |  | 8960 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8984 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.608 |
| walker |  | 9014 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.608 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.609 |
| walker |  | 9062 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.612 |
| walker |  | 9110 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.616 |
| walker |  | 9177 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.619 |
| walker |  | 9185 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.619 |
| walker |  | 9255 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.619 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.615 |
| walker |  | 9357 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.624 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.627 |
| walker |  | 9466 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.627 |
| walker |  | 9503 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 2, sub: 0, line: 23 } |  |  | 0.628 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.637 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.631 |
| walker |  | 9791 | 288 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 9815 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.637 |
| walker |  | 9845 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.639 |
| walker |  | 9883 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.640 |
| walker |  | 9891 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 17, sub: 0, line: 181 } |  |  | 0.640 |
| walker |  | 9931 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.641 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.647 |
| walker |  | 9996 | 65 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.647 |

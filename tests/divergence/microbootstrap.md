Score(3000)=0.553 I=0.836 C=0.366 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.473/0.479/0.638/0.553/0.640/0.589/0.597

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 30 |  | 30 | README tagline: what microbootstrap is, in one sentence | 1.1 |  | 0.000 |
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 36 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 45 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 117 | 72 | Toml::Identity { file: pyproject.toml } |  |  | 0.000 |
| ns | 126 |  | 96 | The complete list of built-in instruments | 1.2 |  | 0.000 |
| walker |  | 189 | 72 | Json::Identity { file: package.json } |  |  | 0.000 |
| ns | 191 |  | 65 | The four bootstrap targets: fastapi, litestar, faststream, or no framework | 1.3 |  | 0.000 |
| ns | 224 |  | 33 | Repository root listing (complete) | 1.4 |  | 0.243 |
| walker |  | 246 | 57 | Fs::DirListing { dir: microbootstrap } |  |  | 0.284 |
| walker |  | 264 | 18 | Fs::DirListing { dir: microbootstrap/middlewares } |  |  | 0.286 |
| walker |  | 287 | 23 | Fs::DirListing { dir: microbootstrap/config } |  |  | 0.294 |
| walker |  | 314 | 27 | Fs::DirListing { dir: microbootstrap/bootstrappers } |  |  | 0.315 |
| ns | 353 |  | 129 | `microbootstrap/` and `microbootstrap/instruments/` listings (complete) | 1.5 |  | 0.267 |
| walker |  | 354 | 40 | Json::Dependencies { file: package.json } |  |  | 0.267 |
| walker |  | 426 | 72 | Fs::DirListing { dir: microbootstrap/instruments } |  |  | 0.455 |
| ns | 439 |  | 86 | `bootstrappers/`, `config/`, `middlewares/`, `examples/` listings (complete) | 1.6 |  | 0.438 |
| walker |  | 444 | 18 | Fs::DirListing { dir: examples } |  |  | 0.480 |
| walker |  | 515 | 71 | Plaintext::DeclSurface { file: Justfile } |  |  | 0.480 |
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.411 |
| walker |  | 774 | 259 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.541 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.473 |
| walker |  | 857 | 83 | Json::Scripts { file: package.json } |  |  | 0.473 |
| walker |  | 904 | 47 | Fs::DirListing { dir: tests } |  |  | 0.473 |
| walker |  | 1003 | 99 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.473 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.415 |
| walker |  | 1228 | 225 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.496 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.476 |
| walker |  | 1266 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.477 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.437 |
| walker |  | 1504 | 238 | Plaintext::Whole { file: Justfile } |  |  | 0.439 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.392 |
| walker |  | 1879 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.680 |
| walker |  | 1943 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.681 |
| walker |  | 1955 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.638 |
| walker |  | 2064 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.638 |
| walker |  | 2114 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 2160 | 46 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 4, sub: 0, line: 31 } |  |  | 0.638 |
| walker |  | 2212 | 52 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.611 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.581 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.552 |
| walker |  | 2906 | 694 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.553 |
| walker |  | 2919 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.536 |
| walker |  | 3082 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.537 |
| walker |  | 3114 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.537 |
| walker |  | 3120 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.537 |
| walker |  | 3129 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.537 |
| walker |  | 3190 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.537 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.516 |
| walker |  | 3211 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.516 |
| walker |  | 3240 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 3292 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.516 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.504 |
| walker |  | 3371 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.504 |
| walker |  | 3416 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 3425 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.504 |
| walker |  | 3441 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.504 |
| walker |  | 3458 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.504 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.483 |
| walker |  | 3621 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 3672 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.516 |
| walker |  | 3682 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.519 |
| walker |  | 3746 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.523 |
| walker |  | 3841 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.545 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.533 |
| walker |  | 3853 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.537 |
| walker |  | 3948 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.573 |
| walker |  | 3960 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.579 |
| walker |  | 4058 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.607 |
| walker |  | 4069 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.613 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.593 |
| walker |  | 4320 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.640 |
| walker |  | 4351 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.640 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.623 |
| walker |  | 4578 | 227 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4740 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4764 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.623 |
| walker |  | 4803 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.623 |
| walker |  | 4843 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.623 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.605 |
| walker |  | 4885 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.605 |
| walker |  | 4901 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.605 |
| walker |  | 4965 | 64 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 2, sub: 0, line: 16 } |  |  | 0.605 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.592 |
| walker |  | 5225 | 260 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.636 |
| walker |  | 5237 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/instrument_box.py, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| walker |  | 5362 | 125 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 1, sub: 0, line: 9 } |  |  | 0.645 |
| walker |  | 5370 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.647 |
| walker |  | 5398 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 3, sub: 0, line: 21 } |  |  | 0.648 |
| walker |  | 5436 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.648 |
| walker |  | 5446 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.649 |
| walker |  | 5466 | 20 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.650 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.630 |
| walker |  | 5581 | 115 | Code::CodeKey { rung: Body, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.630 |
| walker |  | 5594 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5607 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.612 |
| walker |  | 5804 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.612 |
| walker |  | 5835 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 7, sub: 0, line: 57 } |  |  | 0.612 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.607 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.589 |
| walker |  | 6382 | 547 | Toml::Config { file: pyproject.toml } |  |  | 0.589 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.574 |
| walker |  | 6397 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 6489 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.575 |
| walker |  | 6519 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6582 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.575 |
| walker |  | 6590 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.575 |
| walker |  | 6597 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.575 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.560 |
| walker |  | 6724 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.566 |
| walker |  | 6761 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 3, sub: 0, line: 22 } |  |  | 0.567 |
| walker |  | 6791 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 6853 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.571 |
| walker |  | 6861 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.571 |
| walker |  | 6868 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.571 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.557 |
| walker |  | 6982 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.572 |
| walker |  | 7003 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.573 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.558 |
| walker |  | 7216 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.558 |
| walker |  | 7232 | 16 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 7258 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.558 |
| walker |  | 7291 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.552 |
| walker |  | 7383 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.555 |
| walker |  | 7391 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.555 |
| walker |  | 7398 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.555 |
| walker |  | 7406 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.555 |
| walker |  | 7422 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 3, sub: 0, line: 31 } |  |  | 0.556 |
| walker |  | 7573 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.562 |
| walker |  | 7590 | 17 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 7615 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.562 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.546 |
| walker |  | 7806 | 191 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 7829 | 23 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.554 |
| walker |  | 7854 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 3, sub: 0, line: 62 } |  |  | 0.556 |
| walker |  | 7880 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 11, sub: 0, line: 158 } |  |  | 0.557 |
| walker |  | 7910 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 5, sub: 0, line: 73 } |  |  | 0.559 |
| walker |  | 7940 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 7, sub: 0, line: 108 } |  |  | 0.561 |
| walker |  | 7970 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 13, sub: 0, line: 177 } |  |  | 0.563 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.552 |
| walker |  | 8000 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 15, sub: 0, line: 190 } |  |  | 0.554 |
| walker |  | 8052 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 17, sub: 0, line: 199 } |  |  | 0.557 |
| walker |  | 8060 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 19, sub: 0, line: 217 } |  |  | 0.557 |
| walker |  | 8112 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 20, sub: 0, line: 222 } |  |  | 0.560 |
| walker |  | 8193 | 81 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 1, sub: 0, line: 48 } |  |  | 0.569 |
| walker |  | 8246 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.564 |
| walker |  | 8278 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.567 |
| walker |  | 8364 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.570 |
| walker |  | 8372 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.570 |
| walker |  | 8380 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.570 |
| walker |  | 8391 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.572 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.571 |
| walker |  | 8509 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.585 |
| walker |  | 8596 | 87 | Code::CodeKey { rung: Doc, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.585 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.581 |
| walker |  | 8747 | 151 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.581 |
| walker |  | 8779 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 9, sub: 0, line: 73 } |  |  | 0.583 |
| walker |  | 8811 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 11, sub: 0, line: 92 } |  |  | 0.584 |
| walker |  | 8843 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 13, sub: 0, line: 103 } |  |  | 0.586 |
| walker |  | 8894 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 6, sub: 0, line: 57 } |  |  | 0.587 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.593 |
| walker |  | 8948 | 54 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 15, sub: 0, line: 113 } |  |  | 0.595 |
| walker |  | 8956 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.595 |
| walker |  | 9011 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 18, sub: 0, line: 136 } |  |  | 0.597 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.598 |
| walker |  | 9142 | 131 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 2, sub: 0, line: 27 } |  |  | 0.605 |
| walker |  | 9152 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 3, sub: 0, line: 33 } |  |  | 0.605 |
| walker |  | 9164 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 4, sub: 0, line: 41 } |  |  | 0.605 |
| walker |  | 9174 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.605 |
| walker |  | 9255 | 81 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.605 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.602 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.606 |
| walker |  | 9393 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 9417 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.610 |
| walker |  | 9447 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.611 |
| walker |  | 9495 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.614 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.624 |
| walker |  | 9543 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.627 |
| walker |  | 9610 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.630 |
| walker |  | 9618 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.630 |
| walker |  | 9688 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.630 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.637 |
| walker |  | 9790 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.645 |
| walker |  | 9899 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.645 |
| walker |  | 9936 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 2, sub: 0, line: 23 } |  |  | 0.645 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.651 |
| walker |  | 9989 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |

Score(3000)=0.553 I=0.837 C=0.366 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.473/0.479/0.638/0.553/0.640/0.589/0.605

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
| walker |  | 1309 | 238 | Plaintext::Whole { file: Justfile } |  |  | 0.479 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.439 |
| walker |  | 1684 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.761 |
| walker |  | 1748 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.762 |
| walker |  | 1760 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.681 |
| walker |  | 1869 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.681 |
| walker |  | 1919 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.681 |
| walker |  | 1965 | 46 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 4, sub: 0, line: 31 } |  |  | 0.681 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.638 |
| walker |  | 2017 | 52 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.611 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.581 |
| walker |  | 2711 | 694 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.582 |
| walker |  | 2724 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.553 |
| walker |  | 2887 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.553 |
| walker |  | 2919 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.553 |
| walker |  | 2925 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.553 |
| walker |  | 2934 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.553 |
| walker |  | 2995 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.553 |
| walker |  | 3016 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.553 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.537 |
| walker |  | 3045 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 3097 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.537 |
| walker |  | 3176 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.537 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.516 |
| walker |  | 3221 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 3230 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.516 |
| walker |  | 3246 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.516 |
| walker |  | 3263 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.516 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.504 |
| walker |  | 3426 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 3477 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.539 |
| walker |  | 3487 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.542 |
| walker |  | 3551 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.547 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.523 |
| walker |  | 3646 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.545 |
| walker |  | 3658 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.549 |
| walker |  | 3753 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.586 |
| walker |  | 3765 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.592 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.579 |
| walker |  | 3863 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.607 |
| walker |  | 3874 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.613 |
| walker |  | 4125 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.662 |
| walker |  | 4156 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.662 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.640 |
| walker |  | 4383 | 227 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.623 |
| walker |  | 4545 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4569 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.623 |
| walker |  | 4608 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.623 |
| walker |  | 4648 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.623 |
| walker |  | 4690 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.623 |
| walker |  | 4706 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.624 |
| walker |  | 4770 | 64 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 2, sub: 0, line: 16 } |  |  | 0.624 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.605 |
| walker |  | 5030 | 260 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.650 |
| walker |  | 5042 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/instrument_box.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.636 |
| walker |  | 5167 | 125 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 1, sub: 0, line: 9 } |  |  | 0.645 |
| walker |  | 5175 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.647 |
| walker |  | 5203 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 3, sub: 0, line: 21 } |  |  | 0.648 |
| walker |  | 5241 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.648 |
| walker |  | 5251 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.649 |
| walker |  | 5271 | 20 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.650 |
| walker |  | 5386 | 115 | Code::CodeKey { rung: Body, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.650 |
| walker |  | 5399 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 5412 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.630 |
| walker |  | 5609 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.630 |
| walker |  | 5640 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 7, sub: 0, line: 57 } |  |  | 0.630 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.612 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.607 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.589 |
| walker |  | 6187 | 547 | Toml::Config { file: pyproject.toml } |  |  | 0.589 |
| walker |  | 6202 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 6294 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.589 |
| walker |  | 6324 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 6387 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.590 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.575 |
| walker |  | 6395 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.575 |
| walker |  | 6402 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.575 |
| walker |  | 6529 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.582 |
| walker |  | 6566 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 3, sub: 0, line: 22 } |  |  | 0.583 |
| walker |  | 6596 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 6658 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.586 |
| walker |  | 6666 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.586 |
| walker |  | 6673 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.586 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.571 |
| walker |  | 6787 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.587 |
| walker |  | 6808 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.588 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.573 |
| walker |  | 7021 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.573 |
| walker |  | 7037 | 16 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 7063 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.574 |
| walker |  | 7096 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 7188 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.577 |
| walker |  | 7196 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.577 |
| walker |  | 7203 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.577 |
| walker |  | 7211 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.577 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.562 |
| walker |  | 7227 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 3, sub: 0, line: 31 } |  |  | 0.563 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.556 |
| walker |  | 7378 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.562 |
| walker |  | 7395 | 17 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 7420 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.562 |
| walker |  | 7611 | 191 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 7634 | 23 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.563 |
| walker |  | 7659 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 3, sub: 0, line: 62 } |  |  | 0.563 |
| walker |  | 7685 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 11, sub: 0, line: 158 } |  |  | 0.563 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.557 |
| walker |  | 7715 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 5, sub: 0, line: 73 } |  |  | 0.559 |
| walker |  | 7745 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 7, sub: 0, line: 108 } |  |  | 0.561 |
| walker |  | 7775 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 13, sub: 0, line: 177 } |  |  | 0.563 |
| walker |  | 7805 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 15, sub: 0, line: 190 } |  |  | 0.566 |
| walker |  | 7857 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 17, sub: 0, line: 199 } |  |  | 0.569 |
| walker |  | 7865 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 19, sub: 0, line: 217 } |  |  | 0.569 |
| walker |  | 7917 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 20, sub: 0, line: 222 } |  |  | 0.572 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.560 |
| walker |  | 7998 | 81 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 1, sub: 0, line: 48 } |  |  | 0.569 |
| walker |  | 8051 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 8083 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.577 |
| walker |  | 8169 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.580 |
| walker |  | 8177 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.580 |
| walker |  | 8185 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.580 |
| walker |  | 8196 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.581 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.572 |
| walker |  | 8314 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.587 |
| walker |  | 8401 | 87 | Code::CodeKey { rung: Doc, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.587 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.585 |
| walker |  | 8552 | 151 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 8584 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 9, sub: 0, line: 73 } |  |  | 0.590 |
| walker |  | 8616 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 11, sub: 0, line: 92 } |  |  | 0.591 |
| walker |  | 8648 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 13, sub: 0, line: 103 } |  |  | 0.593 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.588 |
| walker |  | 8699 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 6, sub: 0, line: 57 } |  |  | 0.590 |
| walker |  | 8753 | 54 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 15, sub: 0, line: 113 } |  |  | 0.592 |
| walker |  | 8761 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.592 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.589 |
| walker |  | 8816 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 18, sub: 0, line: 136 } |  |  | 0.592 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.597 |
| walker |  | 8947 | 131 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 2, sub: 0, line: 27 } |  |  | 0.604 |
| walker |  | 8957 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 3, sub: 0, line: 33 } |  |  | 0.605 |
| walker |  | 8969 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 4, sub: 0, line: 41 } |  |  | 0.605 |
| walker |  | 8979 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.605 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.605 |
| walker |  | 9060 | 81 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.605 |
| walker |  | 9198 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 9222 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.610 |
| walker |  | 9252 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.610 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.607 |
| walker |  | 9300 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.610 |
| walker |  | 9348 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.613 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.617 |
| walker |  | 9415 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.620 |
| walker |  | 9423 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.620 |
| walker |  | 9493 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.620 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.630 |
| walker |  | 9595 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.638 |
| walker |  | 9704 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.645 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.645 |
| walker |  | 9741 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 2, sub: 0, line: 23 } |  |  | 0.645 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.651 |

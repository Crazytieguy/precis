Score(3000)=0.594 I=0.850 C=0.415 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.475/0.482/0.638/0.594/0.640/0.599/0.616

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
| walker |  | 386 | 72 | Fs::DirListing { dir: microbootstrap/instruments } |  |  | 0.455 |
| walker |  | 404 | 18 | Fs::DirListing { dir: examples } |  |  | 0.470 |
| ns | 439 |  | 86 | `bootstrappers/`, `config/`, `middlewares/`, `examples/` listings (complete) | 1.6 |  | 0.480 |
| walker |  | 444 | 40 | Json::Dependencies { file: package.json } |  |  | 0.480 |
| walker |  | 515 | 71 | Plaintext::DeclSurface { file: Justfile } |  |  | 0.480 |
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.411 |
| walker |  | 774 | 259 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.541 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.473 |
| walker |  | 857 | 83 | Json::Scripts { file: package.json } |  |  | 0.473 |
| walker |  | 904 | 47 | Fs::DirListing { dir: tests } |  |  | 0.473 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.415 |
| walker |  | 1129 | 225 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.496 |
| walker |  | 1167 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.496 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.477 |
| walker |  | 1405 | 238 | Plaintext::Whole { file: Justfile } |  |  | 0.479 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.439 |
| walker |  | 1780 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.761 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.680 |
| walker |  | 1879 | 99 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.680 |
| walker |  | 1943 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.681 |
| walker |  | 1955 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.638 |
| walker |  | 2064 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.638 |
| walker |  | 2114 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 2160 | 46 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 4, sub: 0, line: 31 } |  |  | 0.638 |
| walker |  | 2212 | 52 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 2225 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.611 |
| walker |  | 2388 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.611 |
| walker |  | 2420 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.611 |
| walker |  | 2426 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.611 |
| walker |  | 2435 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.611 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.582 |
| walker |  | 2496 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.582 |
| walker |  | 2517 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.582 |
| walker |  | 2546 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 2598 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.582 |
| walker |  | 2677 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.582 |
| walker |  | 2722 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 2731 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.582 |
| walker |  | 2747 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.582 |
| walker |  | 2764 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.582 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.553 |
| walker |  | 2927 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 2978 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.591 |
| walker |  | 2988 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.594 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.576 |
| walker |  | 3052 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.581 |
| walker |  | 3147 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.605 |
| walker |  | 3159 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.610 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.586 |
| walker |  | 3254 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.625 |
| walker |  | 3266 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.632 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.617 |
| walker |  | 3364 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.648 |
| walker |  | 3375 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.654 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.626 |
| walker |  | 3626 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.675 |
| walker |  | 3657 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.675 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.661 |
| walker |  | 3884 | 227 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4046 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4070 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.661 |
| walker |  | 4109 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.661 |
| walker |  | 4149 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.661 |
| walker |  | 4191 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.661 |
| walker |  | 4207 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.661 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.640 |
| walker |  | 4271 | 64 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 2, sub: 0, line: 16 } |  |  | 0.640 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.622 |
| walker |  | 4531 | 260 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.668 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.649 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.634 |
| walker |  | 5225 | 694 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.636 |
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
| walker |  | 5850 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.607 |
| walker |  | 5942 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.607 |
| walker |  | 5972 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 6035 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.608 |
| walker |  | 6043 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.608 |
| walker |  | 6050 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.608 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.590 |
| walker |  | 6177 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.597 |
| walker |  | 6214 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 3, sub: 0, line: 22 } |  |  | 0.598 |
| walker |  | 6244 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 6306 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.601 |
| walker |  | 6314 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.601 |
| walker |  | 6321 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.601 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.586 |
| walker |  | 6435 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.603 |
| walker |  | 6456 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.604 |
| walker |  | 6669 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.604 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.588 |
| walker |  | 6685 | 16 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 6711 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.588 |
| walker |  | 6744 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 6836 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.592 |
| walker |  | 6844 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.592 |
| walker |  | 6851 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.592 |
| walker |  | 6859 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.592 |
| walker |  | 6875 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 3, sub: 0, line: 31 } |  |  | 0.593 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.578 |
| walker |  | 7026 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.585 |
| walker |  | 7043 | 17 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 7068 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.585 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.569 |
| walker |  | 7259 | 191 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 7282 | 23 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.570 |
| walker |  | 7307 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 3, sub: 0, line: 62 } |  |  | 0.570 |
| walker |  | 7333 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 11, sub: 0, line: 158 } |  |  | 0.570 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.563 |
| walker |  | 7363 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 5, sub: 0, line: 73 } |  |  | 0.563 |
| walker |  | 7393 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 7, sub: 0, line: 108 } |  |  | 0.563 |
| walker |  | 7423 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 13, sub: 0, line: 177 } |  |  | 0.563 |
| walker |  | 7453 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 15, sub: 0, line: 190 } |  |  | 0.563 |
| walker |  | 7505 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 17, sub: 0, line: 199 } |  |  | 0.563 |
| walker |  | 7513 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 19, sub: 0, line: 217 } |  |  | 0.563 |
| walker |  | 7565 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 20, sub: 0, line: 222 } |  |  | 0.564 |
| walker |  | 7646 | 81 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 1, sub: 0, line: 48 } |  |  | 0.564 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.582 |
| walker |  | 7699 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 7731 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.589 |
| walker |  | 7817 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.593 |
| walker |  | 7825 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.593 |
| walker |  | 7833 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.593 |
| walker |  | 7844 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.594 |
| walker |  | 7962 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.610 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.597 |
| walker |  | 8049 | 87 | Code::CodeKey { rung: Doc, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.597 |
| walker |  | 8200 | 151 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 8232 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 9, sub: 0, line: 73 } |  |  | 0.602 |
| walker |  | 8264 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 11, sub: 0, line: 92 } |  |  | 0.603 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.593 |
| walker |  | 8296 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 13, sub: 0, line: 103 } |  |  | 0.594 |
| walker |  | 8347 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 6, sub: 0, line: 57 } |  |  | 0.596 |
| walker |  | 8401 | 54 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 15, sub: 0, line: 113 } |  |  | 0.598 |
| walker |  | 8409 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.598 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.597 |
| walker |  | 8464 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 18, sub: 0, line: 136 } |  |  | 0.599 |
| walker |  | 8595 | 131 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 2, sub: 0, line: 27 } |  |  | 0.606 |
| walker |  | 8605 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 3, sub: 0, line: 33 } |  |  | 0.607 |
| walker |  | 8617 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 4, sub: 0, line: 41 } |  |  | 0.607 |
| walker |  | 8627 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.607 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.602 |
| walker |  | 8708 | 81 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.602 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.599 |
| walker |  | 8846 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 8870 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.604 |
| walker |  | 8900 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.604 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.609 |
| walker |  | 8948 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.612 |
| walker |  | 8996 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.616 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.617 |
| walker |  | 9063 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.620 |
| walker |  | 9071 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.620 |
| walker |  | 9141 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.620 |
| walker |  | 9243 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.629 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.625 |
| walker |  | 9352 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.625 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.628 |
| walker |  | 9389 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 2, sub: 0, line: 23 } |  |  | 0.629 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.638 |
| walker |  | 9677 | 288 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 9701 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.645 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.652 |
| walker |  | 9731 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.654 |
| walker |  | 9769 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.654 |
| walker |  | 9777 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 17, sub: 0, line: 181 } |  |  | 0.654 |
| walker |  | 9817 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.655 |
| walker |  | 9882 | 65 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.655 |
| walker |  | 9952 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.655 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.661 |
| walker |  | 9985 | 33 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.661 |

Score(3000)=0.616 I=0.876 C=0.434 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.473/0.494/0.710/0.616/0.687/0.603/0.619

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
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.410 |
| walker |  | 703 | 259 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.541 |
| walker |  | 786 | 83 | Json::Scripts { file: package.json } |  |  | 0.541 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.473 |
| walker |  | 833 | 47 | Fs::DirListing { dir: tests } |  |  | 0.473 |
| walker |  | 1060 | 227 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.415 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.398 |
| walker |  | 1285 | 225 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.476 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.437 |
| walker |  | 1545 | 260 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.519 |
| walker |  | 1583 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.519 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.464 |
| walker |  | 1958 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.757 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.709 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.679 |
| walker |  | 2267 | 309 | Plaintext::Whole { file: Justfile } |  |  | 0.680 |
| walker |  | 2331 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.681 |
| walker |  | 2343 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2452 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.681 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.648 |
| walker |  | 2502 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.648 |
| walker |  | 2548 | 46 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 4, sub: 0, line: 31 } |  |  | 0.648 |
| walker |  | 2600 | 52 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.648 |
| walker |  | 2613 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 2776 | 163 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.649 |
| walker |  | 2808 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.649 |
| walker |  | 2814 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.649 |
| walker |  | 2823 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.649 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.616 |
| walker |  | 2884 | 61 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.616 |
| walker |  | 2905 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.616 |
| walker |  | 2934 | 29 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 2986 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.616 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.598 |
| walker |  | 3065 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.598 |
| walker |  | 3110 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 3119 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.598 |
| walker |  | 3135 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.598 |
| walker |  | 3152 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.598 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.575 |
| walker |  | 3315 | 163 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.588 |
| walker |  | 3366 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.595 |
| walker |  | 3376 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.597 |
| walker |  | 3440 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.602 |
| walker |  | 3535 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.624 |
| walker |  | 3547 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.628 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.601 |
| walker |  | 3642 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.637 |
| walker |  | 3654 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.642 |
| walker |  | 3752 | 98 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.671 |
| walker |  | 3763 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.677 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.662 |
| walker |  | 4014 | 251 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.709 |
| walker |  | 4045 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.709 |
| walker |  | 4207 | 162 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.687 |
| walker |  | 4231 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.687 |
| walker |  | 4270 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.687 |
| walker |  | 4310 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.687 |
| walker |  | 4352 | 42 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.687 |
| walker |  | 4368 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.687 |
| walker |  | 4432 | 64 | Code::CodeKey { rung: Body, file: microbootstrap/console_writer.py, decl: 2, sub: 0, line: 16 } |  |  | 0.687 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.668 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.649 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.634 |
| walker |  | 5128 | 696 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.636 |
| walker |  | 5140 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/instrument_box.py, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| walker |  | 5265 | 125 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 1, sub: 0, line: 9 } |  |  | 0.645 |
| walker |  | 5273 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.647 |
| walker |  | 5301 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 3, sub: 0, line: 21 } |  |  | 0.648 |
| walker |  | 5339 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.648 |
| walker |  | 5349 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/instrument_box.py, decl: 5, sub: 0, line: 48 } |  |  | 0.649 |
| walker |  | 5369 | 20 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/instrument_box.py, decl: 4, sub: 0, line: 34 } |  |  | 0.650 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.630 |
| walker |  | 5484 | 115 | Code::CodeKey { rung: Body, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.630 |
| walker |  | 5497 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5510 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5707 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.630 |
| walker |  | 5738 | 31 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 7, sub: 0, line: 57 } |  |  | 0.630 |
| walker |  | 5753 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.612 |
| walker |  | 5845 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.613 |
| walker |  | 5875 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/cors_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.607 |
| walker |  | 5938 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 2, sub: 0, line: 18 } |  |  | 0.608 |
| walker |  | 5946 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.608 |
| walker |  | 5953 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 4, sub: 0, line: 27 } |  |  | 0.608 |
| walker |  | 6080 | 127 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/cors_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.615 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.597 |
| walker |  | 6117 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/cors_instrument.py, decl: 3, sub: 0, line: 22 } |  |  | 0.598 |
| walker |  | 6147 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/swagger_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 6209 | 62 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 2, sub: 0, line: 21 } |  |  | 0.601 |
| walker |  | 6217 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.601 |
| walker |  | 6224 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.601 |
| walker |  | 6338 | 114 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/swagger_instrument.py, decl: 1, sub: 0, line: 10 } |  |  | 0.618 |
| walker |  | 6359 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/swagger_instrument.py, decl: 3, sub: 0, line: 25 } |  |  | 0.619 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.604 |
| walker |  | 6572 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.604 |
| walker |  | 6588 | 16 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 6614 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.604 |
| walker |  | 6647 | 33 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.589 |
| walker |  | 6739 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 2, sub: 0, line: 27 } |  |  | 0.592 |
| walker |  | 6747 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.592 |
| walker |  | 6754 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 4, sub: 0, line: 34 } |  |  | 0.592 |
| walker |  | 6762 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 6, sub: 0, line: 52 } |  |  | 0.592 |
| walker |  | 6778 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 3, sub: 0, line: 31 } |  |  | 0.593 |
| walker |  | 6929 | 151 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/pyroscope_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.599 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.585 |
| walker |  | 6946 | 17 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 6971 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.585 |
| walker |  | 7162 | 191 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 7185 | 23 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.585 |
| walker |  | 7210 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 3, sub: 0, line: 62 } |  |  | 0.585 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.570 |
| walker |  | 7236 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 11, sub: 0, line: 158 } |  |  | 0.570 |
| walker |  | 7266 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 5, sub: 0, line: 73 } |  |  | 0.570 |
| walker |  | 7296 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 7, sub: 0, line: 108 } |  |  | 0.570 |
| walker |  | 7326 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 13, sub: 0, line: 177 } |  |  | 0.570 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.563 |
| walker |  | 7356 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 15, sub: 0, line: 190 } |  |  | 0.563 |
| walker |  | 7408 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 17, sub: 0, line: 199 } |  |  | 0.563 |
| walker |  | 7416 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 19, sub: 0, line: 217 } |  |  | 0.563 |
| walker |  | 7468 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 20, sub: 0, line: 222 } |  |  | 0.564 |
| walker |  | 7549 | 81 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/litestar.py, decl: 1, sub: 0, line: 48 } |  |  | 0.564 |
| walker |  | 7602 | 53 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/health_checks_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 7634 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 1, sub: 0, line: 8 } |  |  | 0.572 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.589 |
| walker |  | 7720 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 3, sub: 0, line: 26 } |  |  | 0.593 |
| walker |  | 7728 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.593 |
| walker |  | 7736 | 8 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 6, sub: 0, line: 40 } |  |  | 0.593 |
| walker |  | 7747 | 11 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/health_checks_instrument.py, decl: 5, sub: 0, line: 37 } |  |  | 0.594 |
| walker |  | 7865 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/health_checks_instrument.py, decl: 2, sub: 0, line: 14 } |  |  | 0.610 |
| walker |  | 7952 | 87 | Code::CodeKey { rung: Doc, file: microbootstrap/bootstrappers/litestar.py, decl: 10, sub: 0, line: 133 } |  |  | 0.610 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.597 |
| walker |  | 8103 | 151 | Code::CodeKey { rung: Names, file: microbootstrap/bootstrappers/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 8135 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 9, sub: 0, line: 73 } |  |  | 0.602 |
| walker |  | 8167 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 11, sub: 0, line: 92 } |  |  | 0.603 |
| walker |  | 8199 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 13, sub: 0, line: 103 } |  |  | 0.605 |
| walker |  | 8250 | 51 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 6, sub: 0, line: 57 } |  |  | 0.607 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.596 |
| walker |  | 8304 | 54 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 15, sub: 0, line: 113 } |  |  | 0.598 |
| walker |  | 8312 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.598 |
| walker |  | 8367 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 18, sub: 0, line: 136 } |  |  | 0.601 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.599 |
| walker |  | 8498 | 131 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 2, sub: 0, line: 27 } |  |  | 0.606 |
| walker |  | 8508 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 3, sub: 0, line: 33 } |  |  | 0.607 |
| walker |  | 8520 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/bootstrappers/fastapi.py, decl: 4, sub: 0, line: 41 } |  |  | 0.607 |
| walker |  | 8530 | 10 | Code::CodeKey { rung: Body, file: microbootstrap/bootstrappers/fastapi.py, decl: 17, sub: 0, line: 131 } |  |  | 0.607 |
| walker |  | 8611 | 81 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.607 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.602 |
| walker |  | 8749 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.603 |
| walker |  | 8773 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.604 |
| walker |  | 8803 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.604 |
| walker |  | 8851 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.607 |
| walker |  | 8899 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.611 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.616 |
| walker |  | 8966 | 67 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.619 |
| walker |  | 8974 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 11, sub: 0, line: 69 } |  |  | 0.619 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.620 |
| walker |  | 9044 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.620 |
| walker |  | 9146 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.629 |
| walker |  | 9255 | 109 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.629 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.625 |
| walker |  | 9292 | 37 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 2, sub: 0, line: 23 } |  |  | 0.625 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.629 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.638 |
| walker |  | 9580 | 288 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 9604 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.645 |
| walker |  | 9634 | 30 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.647 |
| walker |  | 9672 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.648 |
| walker |  | 9680 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 17, sub: 0, line: 181 } |  |  | 0.648 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.654 |
| walker |  | 9720 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.655 |
| walker |  | 9785 | 65 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.655 |
| walker |  | 9855 | 70 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.655 |
| walker |  | 9947 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.655 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.661 |
| walker |  | 9997 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 10, sub: 0, line: 99 } |  |  | 0.665 |

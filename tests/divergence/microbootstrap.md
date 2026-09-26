Score(3000)=0.774 I=0.911 C=0.658 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.493/0.744/0.674/0.774/0.639/0.536/0.558

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 30 |  | 30 | README tagline: what microbootstrap is, in one sentence | 1.1 |  | 0.000 |
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 74 | 41 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 126 |  | 96 | The complete list of built-in instruments | 1.2 |  | 0.246 |
| walker |  | 146 | 72 | Toml::Identity { file: pyproject.toml } |  |  | 0.246 |
| ns | 191 |  | 65 | The four bootstrap targets: fastapi, litestar, faststream, or no framework | 1.3 |  | 0.176 |
| walker |  | 218 | 72 | Json::Identity { file: package.json } |  |  | 0.176 |
| ns | 224 |  | 33 | Repository root listing (complete) | 1.4 |  | 0.429 |
| walker |  | 275 | 57 | Fs::DirListing { dir: microbootstrap } |  |  | 0.456 |
| walker |  | 293 | 18 | Fs::DirListing { dir: microbootstrap/middlewares } |  |  | 0.457 |
| walker |  | 316 | 23 | Fs::DirListing { dir: microbootstrap/config } |  |  | 0.463 |
| walker |  | 343 | 27 | Fs::DirListing { dir: microbootstrap/bootstrappers } |  |  | 0.478 |
| ns | 353 |  | 129 | `microbootstrap/` and `microbootstrap/instruments/` listings (complete) | 1.5 |  | 0.398 |
| walker |  | 415 | 72 | Fs::DirListing { dir: microbootstrap/instruments } |  |  | 0.610 |
| walker |  | 426 | 11 | Fs::DirListing { dir: .github/workflows } |  |  | 0.610 |
| ns | 439 |  | 86 | `bootstrappers/`, `config/`, `middlewares/`, `examples/` listings (complete) | 1.6 |  | 0.586 |
| ns | 641 |  | 202 | `microbootstrap/__init__.py` — the complete `__all__` export block | 1.7 |  | 0.501 |
| walker |  | 735 | 309 | Plaintext::Whole { file: Justfile } |  |  | 0.503 |
| walker |  | 775 | 40 | Json::Dependencies { file: package.json } |  |  | 0.503 |
| walker |  | 793 | 18 | Fs::DirListing { dir: examples } |  |  | 0.543 |
| ns | 824 |  | 183 | README canonical usage snippet: settings class -> bootstrapper -> application | 1.8 |  | 0.475 |
| walker |  | 876 | 83 | Json::Scripts { file: package.json } |  |  | 0.475 |
| walker |  | 923 | 47 | Fs::DirListing { dir: tests } |  |  | 0.475 |
| ns | 1115 |  | 291 | README section map: every `##`/`###`/`####` heading location | 1.9 |  | 0.416 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.400 |
| walker |  | 1298 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.744 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.683 |
| walker |  | 1577 | 279 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 1804 | 227 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.760 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.679 |
| walker |  | 1842 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.680 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.637 |
| walker |  | 2040 | 198 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 2084 | 44 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.684 |
| walker |  | 2148 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.691 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.662 |
| walker |  | 2236 | 88 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.688 |
| walker |  | 2324 | 88 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.730 |
| walker |  | 2415 | 91 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.763 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.726 |
| walker |  | 2659 | 244 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.787 |
| walker |  | 2669 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.793 |
| walker |  | 2680 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.800 |
| walker |  | 2692 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.807 |
| walker |  | 2704 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.814 |
| walker |  | 2722 | 18 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.814 |
| walker |  | 2786 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.815 |
| walker |  | 2799 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.774 |
| walker |  | 2993 | 194 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.774 |
| walker |  | 3011 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.774 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.751 |
| walker |  | 3055 | 44 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.751 |
| walker |  | 3061 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.751 |
| walker |  | 3070 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.751 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.722 |
| walker |  | 3283 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.722 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.705 |
| walker |  | 3498 | 215 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.706 |
| walker |  | 3510 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.706 |
| walker |  | 3536 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.706 |
| walker |  | 3562 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.706 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.676 |
| walker |  | 3590 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.676 |
| walker |  | 3612 | 22 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3722 | 110 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.676 |
| walker |  | 3761 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.676 |
| walker |  | 3805 | 44 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3842 | 37 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.676 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.661 |
| walker |  | 3921 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.661 |
| walker |  | 3966 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 3975 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.661 |
| walker |  | 3991 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.661 |
| walker |  | 4008 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.661 |
| walker |  | 4031 | 23 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4190 | 159 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 0, line: 20 } |  |  | 0.661 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.639 |
| walker |  | 4400 | 210 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 1, line: 20 } |  |  | 0.639 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.622 |
| walker |  | 4569 | 169 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 2, line: 20 } |  |  | 0.622 |
| walker |  | 4731 | 162 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 3, line: 20 } |  |  | 0.622 |
| walker |  | 4754 | 23 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.604 |
| walker |  | 4941 | 187 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.604 |
| walker |  | 4966 | 25 | Code::CodeKey { rung: Doc, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.604 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.591 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.572 |
| walker |  | 5662 | 696 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.573 |
| walker |  | 5687 | 25 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 5769 | 82 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.573 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.557 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.552 |
| walker |  | 5974 | 205 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.552 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.536 |
| walker |  | 6228 | 254 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 6258 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 6270 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.536 |
| walker |  | 6300 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 6312 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.536 |
| walker |  | 6328 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.537 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.523 |
| walker |  | 6427 | 99 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 6444 | 17 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 3, sub: 0, line: 19 } |  |  | 0.530 |
| walker |  | 6664 | 220 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 4, sub: 0, line: 23 } |  |  | 0.562 |
| walker |  | 6681 | 17 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 5, sub: 0, line: 29 } |  |  | 0.564 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.548 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.535 |
| walker |  | 6997 | 316 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 7021 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.540 |
| walker |  | 7053 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.541 |
| walker |  | 7099 | 46 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.541 |
| walker |  | 7152 | 53 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.542 |
| walker |  | 7204 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.542 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.528 |
| walker |  | 7259 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.528 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.521 |
| walker |  | 7351 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.522 |
| walker |  | 7444 | 93 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 10, sub: 0, line: 99 } |  |  | 0.524 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.509 |
| walker |  | 7802 | 358 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 4, sub: 0, line: 48 } |  |  | 0.537 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.526 |
| walker |  | 8047 | 245 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/logging_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 8065 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 8, sub: 0, line: 92 } |  |  | 0.535 |
| walker |  | 8115 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 9, sub: 0, line: 97 } |  |  | 0.536 |
| walker |  | 8168 | 53 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 4, sub: 0, line: 36 } |  |  | 0.537 |
| walker |  | 8263 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 10, sub: 0, line: 98 } |  |  | 0.537 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.528 |
| walker |  | 8381 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 6, sub: 0, line: 76 } |  |  | 0.529 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.528 |
| walker |  | 8531 | 150 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 14, sub: 0, line: 148 } |  |  | 0.534 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.530 |
| walker |  | 8746 | 215 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 12, sub: 0, line: 127 } |  |  | 0.543 |
| walker |  | 8752 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 9, sub: 0, line: 50 } |  |  | 0.544 |
| walker |  | 8758 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 10, sub: 0, line: 53 } |  |  | 0.545 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.543 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.549 |
| walker |  | 8977 | 219 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/sentry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 9002 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 6, sub: 0, line: 67 } |  |  | 0.559 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.560 |
| walker |  | 9088 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 8, sub: 0, line: 89 } |  |  | 0.563 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.560 |
| walker |  | 9341 | 253 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.574 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.578 |
| walker |  | 9489 | 148 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 9513 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.585 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.596 |
| walker |  | 9561 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.599 |
| walker |  | 9609 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 5, sub: 0, line: 35 } |  |  | 0.600 |
| walker |  | 9657 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.603 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.610 |
| walker |  | 9709 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 7, sub: 0, line: 46 } |  |  | 0.610 |
| walker |  | 9784 | 75 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 9, sub: 0, line: 60 } |  |  | 0.612 |
| walker |  | 9883 | 99 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 6, sub: 0, line: 37 } |  |  | 0.612 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.619 |
| walker |  | 9985 | 102 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 4, sub: 0, line: 28 } |  |  | 0.627 |

Score(3000)=0.767 I=0.924 C=0.637 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.475/0.694/0.710/0.767/0.686/0.576/0.567

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
| walker |  | 1116 | 193 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.416 |
| ns | 1241 |  | 126 | `settings.py` roster: env-prefix constants and all six class names | 2.1 |  | 0.400 |
| walker |  | 1491 | 375 | Markdown::Prelude { file: README.md } |  |  | 0.744 |
| ns | 1492 |  | 251 | `BaseServiceSettings`: all five service fields and the env-sourcing `model_config` | 2.2 | 2.1 | 0.683 |
| walker |  | 1785 | 294 | Code::CodeKey { rung: Names, file: microbootstrap/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.774 |
| ns | 1806 |  | 314 | `ServerConfig` fields; `LitestarSettings` and `FastApiSettings` mixin lists | 2.3 | 2.1 | 0.692 |
| ns | 2007 |  | 201 | `FastStreamSettings` and `InstrumentsSetupperSettings` mixin lists | 2.4 | 2.1 | 0.648 |
| walker |  | 2012 | 227 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.710 |
| walker |  | 2223 | 211 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.710 |
| ns | 2228 |  | 221 | README Settings section: env sourcing and `ENVIRONMENT_PREFIX` | 2.5 |  | 0.680 |
| walker |  | 2261 | 38 | Fs::DirListing { dir: tests/bootstrappers } |  |  | 0.680 |
| walker |  | 2459 | 198 | Code::CodeKey { rung: Names, file: microbootstrap/settings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| ns | 2478 |  | 250 | `instruments/base.py`: `BaseInstrumentConfig`, `Instrument` header, complete method roster | 3.1 |  | 0.680 |
| walker |  | 2503 | 44 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.688 |
| walker |  | 2567 | 64 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 5, sub: 0, line: 53 } |  |  | 0.695 |
| walker |  | 2655 | 88 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.719 |
| walker |  | 2743 | 88 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.758 |
| walker |  | 2834 | 91 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.788 |
| ns | 2842 |  | 364 | Instrument roster: every instrument class with its `instrument_name` and `ready_condition` | 3.2 |  | 0.748 |
| walker |  | 2844 | 10 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 9, sub: 0, line: 105 } |  |  | 0.754 |
| ns | 3039 |  | 197 | `InstrumentBox`: registry fields, `initialize`, and remaining member roster | 3.3 |  | 0.732 |
| walker |  | 3088 | 244 | Code::CodeKey { rung: Decl, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.786 |
| walker |  | 3099 | 11 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 8, sub: 0, line: 90 } |  |  | 0.792 |
| walker |  | 3111 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 6, sub: 0, line: 60 } |  |  | 0.799 |
| walker |  | 3123 | 12 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 7, sub: 0, line: 75 } |  |  | 0.805 |
| walker |  | 3141 | 18 | Code::CodeKey { rung: Doc, file: microbootstrap/settings.py, decl: 4, sub: 0, line: 29 } |  |  | 0.805 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.773 |
| walker |  | 3205 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.775 |
| walker |  | 3218 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.757 |
| walker |  | 3412 | 194 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.757 |
| walker |  | 3430 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.757 |
| walker |  | 3474 | 44 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.757 |
| walker |  | 3480 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.757 |
| walker |  | 3489 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.757 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.725 |
| walker |  | 3702 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.725 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.709 |
| walker |  | 3917 | 215 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3929 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.709 |
| walker |  | 3955 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.709 |
| walker |  | 3981 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.709 |
| walker |  | 4009 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.710 |
| walker |  | 4031 | 22 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 4141 | 110 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.710 |
| walker |  | 4180 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.710 |
| walker |  | 4224 | 44 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.686 |
| walker |  | 4261 | 37 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.686 |
| walker |  | 4340 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.686 |
| walker |  | 4385 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 4394 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.686 |
| walker |  | 4410 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.687 |
| walker |  | 4427 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.687 |
| walker |  | 4452 | 25 | Code::CodeKey { rung: Doc, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.687 |
| walker |  | 4475 | 23 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.668 |
| walker |  | 4634 | 159 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 0, line: 20 } |  |  | 0.668 |
| walker |  | 4844 | 210 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 1, line: 20 } |  |  | 0.648 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.648 |
| walker |  | 5013 | 169 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 2, line: 20 } |  |  | 0.648 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.634 |
| walker |  | 5175 | 162 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 3, line: 20 } |  |  | 0.634 |
| walker |  | 5198 | 23 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 5385 | 187 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.634 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.614 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.597 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.591 |
| walker |  | 6081 | 696 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.592 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.575 |
| walker |  | 6106 | 25 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6188 | 82 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.576 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.561 |
| walker |  | 6393 | 205 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.561 |
| walker |  | 6647 | 254 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.561 |
| walker |  | 6663 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.562 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.547 |
| walker |  | 6693 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6705 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.547 |
| walker |  | 6735 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6747 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.547 |
| walker |  | 6768 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.547 |
| walker |  | 6867 | 99 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 6884 | 17 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 3, sub: 0, line: 19 } |  |  | 0.553 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.540 |
| walker |  | 7104 | 220 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 4, sub: 0, line: 23 } |  |  | 0.569 |
| walker |  | 7121 | 17 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 5, sub: 0, line: 29 } |  |  | 0.571 |
| walker |  | 7127 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 9, sub: 0, line: 50 } |  |  | 0.572 |
| walker |  | 7133 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 10, sub: 0, line: 53 } |  |  | 0.573 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.558 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.551 |
| walker |  | 7449 | 316 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 7473 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.555 |
| walker |  | 7505 | 32 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.557 |
| walker |  | 7551 | 46 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.557 |
| walker |  | 7604 | 53 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.558 |
| walker |  | 7656 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.558 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.542 |
| walker |  | 7711 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.542 |
| walker |  | 7803 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.542 |
| walker |  | 7896 | 93 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 10, sub: 0, line: 99 } |  |  | 0.545 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.533 |
| walker |  | 8254 | 358 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 4, sub: 0, line: 48 } |  |  | 0.560 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.550 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.550 |
| walker |  | 8499 | 245 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/logging_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 8517 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 8, sub: 0, line: 92 } |  |  | 0.559 |
| walker |  | 8567 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 9, sub: 0, line: 97 } |  |  | 0.559 |
| walker |  | 8620 | 53 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 4, sub: 0, line: 36 } |  |  | 0.560 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.556 |
| walker |  | 8715 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 10, sub: 0, line: 98 } |  |  | 0.556 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.553 |
| walker |  | 8833 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 6, sub: 0, line: 76 } |  |  | 0.554 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.560 |
| walker |  | 8983 | 150 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 14, sub: 0, line: 148 } |  |  | 0.567 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.568 |
| walker |  | 9198 | 215 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 12, sub: 0, line: 127 } |  |  | 0.581 |
| walker |  | 9205 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 11, sub: 0, line: 56 } |  |  | 0.582 |
| walker |  | 9222 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/base.py, decl: 11, sub: 0, line: 56 } |  |  | 0.585 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.582 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.586 |
| walker |  | 9441 | 219 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/sentry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 9466 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 6, sub: 0, line: 67 } |  |  | 0.595 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.606 |
| walker |  | 9552 | 86 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 8, sub: 0, line: 89 } |  |  | 0.608 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.616 |
| walker |  | 9805 | 253 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.629 |
| walker |  | 9953 | 148 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.641 |
| walker |  | 9977 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.642 |
| walker |  | 9991 | 14 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.643 |

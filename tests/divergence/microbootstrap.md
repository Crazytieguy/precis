Score(3000)=0.767 I=0.924 C=0.637 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.475/0.694/0.710/0.767/0.686/0.575/0.568

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
| walker |  | 3187 | 64 | Fs::DirListing { dir: tests/instruments } |  |  | 0.806 |
| ns | 3198 |  | 159 | `Instrument` abstract methods and default hook implementations (bodies) | 3.4 | 3.1 | 0.775 |
| walker |  | 3200 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/instruments_setupper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| ns | 3319 |  | 121 | `Instrument.configure_instrument` and `write_status` (bodies) | 3.5 | 3.1 | 0.757 |
| walker |  | 3386 | 186 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 1, sub: 0, line: 19 } |  |  | 0.757 |
| walker |  | 3404 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 4, sub: 0, line: 32 } |  |  | 0.757 |
| walker |  | 3456 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments_setupper.py, decl: 5, sub: 0, line: 40 } |  |  | 0.757 |
| walker |  | 3462 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 8, sub: 0, line: 62 } |  |  | 0.757 |
| walker |  | 3471 | 9 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 9, sub: 0, line: 65 } |  |  | 0.757 |
| ns | 3579 |  | 260 | `InstrumentBox` bodies: config dispatch and the replace-on-register rule | 3.6 | 3.3 | 0.725 |
| walker |  | 3684 | 213 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.725 |
| ns | 3848 |  | 269 | `SentryConfig`: complete field set | 4.1 |  | 0.709 |
| walker |  | 3899 | 215 | Code::CodeKey { rung: Names, file: microbootstrap/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3911 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 8, sub: 0, line: 100 } |  |  | 0.709 |
| walker |  | 3937 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 5, sub: 0, line: 48 } |  |  | 0.709 |
| walker |  | 3963 | 26 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 6, sub: 0, line: 60 } |  |  | 0.709 |
| walker |  | 3991 | 28 | Code::CodeKey { rung: Decl, file: microbootstrap/helpers.py, decl: 4, sub: 0, line: 35 } |  |  | 0.710 |
| walker |  | 4003 | 12 | Code::CodeKey { rung: Names, file: microbootstrap/console_writer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 4123 | 120 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 1, sub: 0, line: 10 } |  |  | 0.710 |
| walker |  | 4162 | 39 | Code::CodeKey { rung: Decl, file: microbootstrap/console_writer.py, decl: 3, sub: 0, line: 22 } |  |  | 0.710 |
| walker |  | 4206 | 44 | Code::CodeKey { rung: Names, file: microbootstrap/granian_server.py, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| ns | 4224 |  | 376 | `OpentelemetryConfig`: complete field set | 4.2 |  | 0.686 |
| walker |  | 4243 | 37 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 2, sub: 0, line: 27 } |  |  | 0.686 |
| walker |  | 4322 | 79 | Code::CodeKey { rung: Decl, file: microbootstrap/granian_server.py, decl: 1, sub: 0, line: 16 } |  |  | 0.686 |
| walker |  | 4367 | 45 | Code::CodeKey { rung: Names, file: microbootstrap/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 4376 | 9 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.686 |
| walker |  | 4392 | 16 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 2, sub: 0, line: 5 } |  |  | 0.687 |
| walker |  | 4409 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/exceptions.py, decl: 3, sub: 0, line: 9 } |  |  | 0.687 |
| ns | 4499 |  | 275 | `LoggingConfig`: complete field set plus the exclude-endpoint validator | 4.3 |  | 0.668 |
| ns | 4844 |  | 345 | Prometheus config family: `BasePrometheusConfig` and its three framework variants | 4.4 |  | 0.648 |
| walker |  | 5105 | 696 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.650 |
| ns | 5111 |  | 267 | `CorsConfig` and `SwaggerConfig`: complete field sets | 4.5 |  | 0.635 |
| walker |  | 5310 | 205 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.635 |
| walker |  | 5323 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 5470 |  | 359 | `HealthCheckTypedDict` + `HealthChecksConfig`, and `PyroscopeConfig` | 4.6 |  | 0.615 |
| walker |  | 5492 | 169 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 0, line: 20 } |  |  | 0.615 |
| walker |  | 5702 | 210 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 1, line: 20 } |  |  | 0.615 |
| ns | 5790 |  | 320 | Readiness predicates: `is_ready` for all eight instruments | 4.7 |  | 0.598 |
| walker |  | 5871 | 169 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 2, line: 20 } |  |  | 0.598 |
| ns | 5885 |  | 95 | FastStream broker-middleware protocols and `FastStreamOpentelemetryConfig` | 4.8 |  | 0.592 |
| walker |  | 6033 | 162 | Code::CodeKey { rung: Decl, file: microbootstrap/config/fastapi.py, decl: 1, sub: 3, line: 20 } |  |  | 0.592 |
| walker |  | 6046 | 13 | Code::CodeKey { rung: Names, file: microbootstrap/config/faststream.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| ns | 6104 |  | 219 | Instrument method roster: every `bootstrap` / `teardown` / private override, by line | 4.9 |  | 0.575 |
| walker |  | 6243 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/config/faststream.py, decl: 1, sub: 0, line: 20 } |  |  | 0.575 |
| ns | 6391 |  | 287 | Instrument module-level symbol roster: helpers not attached to any class | 4.10 |  | 0.561 |
| walker |  | 6497 | 254 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.561 |
| walker |  | 6513 | 16 | Code::CodeKey { rung: Body, file: microbootstrap/helpers.py, decl: 7, sub: 0, line: 96 } |  |  | 0.561 |
| walker |  | 6528 | 15 | Code::CodeKey { rung: Names, file: microbootstrap/config/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6620 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/config/litestar.py, decl: 1, sub: 0, line: 13 } |  |  | 0.562 |
| walker |  | 6650 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/litestar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6662 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/litestar.py, decl: 1, sub: 0, line: 14 } |  |  | 0.562 |
| ns | 6684 |  | 293 | `ApplicationBootstrapper`: class attributes and complete member roster | 5.1 |  | 0.547 |
| walker |  | 6692 | 30 | Code::CodeKey { rung: Names, file: microbootstrap/middlewares/fastapi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6704 | 12 | Code::CodeKey { rung: Decl, file: microbootstrap/middlewares/fastapi.py, decl: 1, sub: 0, line: 12 } |  |  | 0.547 |
| walker |  | 6725 | 21 | Code::CodeKey { rung: Body, file: microbootstrap/instruments_setupper.py, decl: 3, sub: 0, line: 28 } |  |  | 0.547 |
| walker |  | 6814 | 89 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 6833 | 19 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 3, sub: 0, line: 19 } |  |  | 0.552 |
| ns | 6945 |  | 261 | `ApplicationBootstrapper.bootstrap()` — the whole application-assembly pipeline | 5.2 | 5.1 | 0.539 |
| walker |  | 7030 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 4, sub: 0, line: 23 } |  |  | 0.566 |
| walker |  | 7040 | 10 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 7, sub: 0, line: 42 } |  |  | 0.567 |
| walker |  | 7056 | 16 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 8, sub: 0, line: 45 } |  |  | 0.569 |
| walker |  | 7073 | 17 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/base.py, decl: 5, sub: 0, line: 29 } |  |  | 0.570 |
| walker |  | 7079 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 9, sub: 0, line: 50 } |  |  | 0.571 |
| walker |  | 7085 | 6 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 10, sub: 0, line: 53 } |  |  | 0.573 |
| ns | 7214 |  | 269 | `ApplicationBootstrapper.__init__` and the `configure_*` fluent methods | 5.3 | 5.1 | 0.557 |
| ns | 7344 |  | 130 | `use_instrument` decorator factory and `ApplicationBootstrapper.teardown` | 5.4 | 5.1 | 0.550 |
| walker |  | 7373 | 288 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 7397 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 8, sub: 0, line: 91 } |  |  | 0.553 |
| walker |  | 7435 | 38 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 15, sub: 0, line: 170 } |  |  | 0.554 |
| walker |  | 7443 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 17, sub: 0, line: 181 } |  |  | 0.554 |
| walker |  | 7483 | 40 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 3, sub: 0, line: 42 } |  |  | 0.555 |
| walker |  | 7546 | 63 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 5, sub: 0, line: 72 } |  |  | 0.557 |
| walker |  | 7598 | 52 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 7, sub: 0, line: 82 } |  |  | 0.557 |
| walker |  | 7653 | 55 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 6, sub: 0, line: 74 } |  |  | 0.557 |
| ns | 7696 |  | 352 | `bootstrappers/litestar.py` roster: bootstrapper, every registered instrument, every helper | 5.5 |  | 0.541 |
| walker |  | 7745 | 92 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 22, sub: 0, line: 197 } |  |  | 0.542 |
| walker |  | 7838 | 93 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 10, sub: 0, line: 99 } |  |  | 0.544 |
| ns | 7978 |  | 282 | `bootstrappers/fastapi.py` roster: bootstrapper and every registered instrument | 5.6 |  | 0.533 |
| walker |  | 8196 | 358 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/opentelemetry_instrument.py, decl: 4, sub: 0, line: 48 } |  |  | 0.559 |
| ns | 8270 |  | 292 | `bootstrappers/faststream.py` roster: bootstrapper, loggers, every registered instrument | 5.7 |  | 0.550 |
| walker |  | 8441 | 245 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/logging_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 8459 | 18 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 8, sub: 0, line: 92 } |  |  | 0.559 |
| ns | 8460 |  | 190 | `InstrumentsSetupper`: member roster and the four instruments it registers | 5.8 |  | 0.558 |
| walker |  | 8509 | 50 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 9, sub: 0, line: 97 } |  |  | 0.559 |
| walker |  | 8562 | 53 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 4, sub: 0, line: 36 } |  |  | 0.559 |
| walker |  | 8657 | 95 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 10, sub: 0, line: 98 } |  |  | 0.559 |
| ns | 8692 |  | 232 | README Configuration: simple values overwrite, complex values merge | 5.9 |  | 0.555 |
| ns | 8771 |  | 79 | `LitestarBootstrapper.bootstrap_before`: how the console table and teardown get attached | 5.10 |  | 0.552 |
| walker |  | 8775 | 118 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 6, sub: 0, line: 76 } |  |  | 0.553 |
| ns | 8914 |  | 143 | `helpers.py`: complete function roster and `VALID_PATH_PATTERN` | 6.1 |  | 0.560 |
| walker |  | 8917 | 142 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 14, sub: 0, line: 148 } |  |  | 0.566 |
| walker |  | 8925 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 21, sub: 0, line: 212 } |  |  | 0.566 |
| ns | 9021 |  | 107 | `exceptions.py`: the complete exception hierarchy | 6.2 |  | 0.568 |
| walker |  | 9122 | 197 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 12, sub: 0, line: 127 } |  |  | 0.578 |
| walker |  | 9135 | 13 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/logging_instrument.py, decl: 13, sub: 0, line: 140 } |  |  | 0.580 |
| walker |  | 9142 | 7 | Code::CodeKey { rung: Body, file: microbootstrap/instruments/base.py, decl: 11, sub: 0, line: 56 } |  |  | 0.580 |
| walker |  | 9159 | 17 | Code::CodeKey { rung: Doc, file: microbootstrap/instruments/base.py, decl: 11, sub: 0, line: 56 } |  |  | 0.583 |
| ns | 9269 |  | 248 | Middleware builders and `create_granian_server`: signatures and returned classes | 6.3 |  | 0.580 |
| ns | 9376 |  | 107 | `config/litestar.py`: the `LitestarConfig` application-config dataclass | 6.4 |  | 0.585 |
| walker |  | 9378 | 219 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/sentry_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 9403 | 25 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 6, sub: 0, line: 67 } |  |  | 0.593 |
| walker |  | 9481 | 78 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 8, sub: 0, line: 89 } |  |  | 0.596 |
| walker |  | 9489 | 8 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 11, sub: 0, line: 122 } |  |  | 0.596 |
| ns | 9534 |  | 158 | Test tree listings (complete) and `.github/workflows/` | 7.1 |  | 0.607 |
| ns | 9704 |  | 170 | `Justfile`: install, lint, lint-ci and test recipes | 7.2 |  | 0.615 |
| walker |  | 9742 | 253 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/sentry_instrument.py, decl: 1, sub: 0, line: 15 } |  |  | 0.628 |
| walker |  | 9880 | 138 | Code::CodeKey { rung: Names, file: microbootstrap/instruments/prometheus_instrument.py, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 9904 | 24 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 3, sub: 0, line: 24 } |  |  | 0.633 |
| walker |  | 9952 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 2, sub: 0, line: 17 } |  |  | 0.635 |
| ns | 9954 |  | 250 | `pyproject.toml`: identity, python requirement and optional-dependency extras | 7.3 |  | 0.642 |
| walker |  | 10000 | 48 | Code::CodeKey { rung: Decl, file: microbootstrap/instruments/prometheus_instrument.py, decl: 8, sub: 0, line: 55 } |  |  | 0.645 |

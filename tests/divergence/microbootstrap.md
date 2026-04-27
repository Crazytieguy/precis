scores: Sim=0.381 Reached=20/50 Early=4 Late=12 Partial=11 Missing=19 Used=9881/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 4 | 0 | 5 | 0.43 |
| 2 | 14 | 11 | 2 | 1 | 0.86 |
| 3 | 7 | 3 | 0 | 4 | 0.54 |
| 4 | 12 | 0 | 7 | 5 | 0.43 |
| 5 | 8 | 2 | 2 | 4 | 0.40 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 26 | — | — | 0.00 | missing | README one-liner statement of purpose |  |
| 1.2 | 59 | 33 | -26 | 1.00 | early | Repo top-level layout |  |
| 1.3 | 158 | — | — | 0.00 | missing | README — list of supported instruments |  |
| 1.4 | 223 | — | — | 0.00 | missing | README — list of target frameworks |  |
| 1.5 | 315 | 613 | +298 | 1.00 | late | Package top-level files + small subdirs |  |
| 1.6 | 410 | 1518 | +1108 | 1.00 | late | Bootstrappers + instruments subdir contents |  |
| 1.7 | 610 | — | — | 0.00 | missing | Public API re-exports (__all__) |  |
| 1.8 | 795 | — | — | 0.00 | missing | README — canonical Litestar quickstart snippet |  |
| 1.9 | 1027 | 335 | -692 | 0.89 | early | README table-of-contents (H2/H3 outline) | README headline in README.md (t=335, 17 atoms) |
| 2.1 | 1093 | 2112 | +1019 | 1.00 | late | Instrument ABC — class declaration | python class body at microbootstrap/instruments/base.py:23 (t=2112, 3 atoms) |
| 2.2 | 1126 | 2074 | +948 | 1.00 | late | BaseInstrumentConfig (pydantic base) | python decl names surface in microbootstrap/instruments/base.py (t=2041, 2 atoms) |
| 2.3 | 1201 | 6588 | +5387 | 0.86 | late | Instrument ABC — abstract methods (signatures) | python method sigs in microbootstrap/instruments/base.py (t=6453, 4 atoms) |
| 2.4 | 1345 | — | — | 0.77 | partial | Instrument ABC — overridable hook signatures | python method sigs in microbootstrap/instruments/base.py (t=6453, 8 atoms) |
| 2.5 | 1527 | 7832 | +6305 | 1.00 | late | BaseServiceSettings — service_* fields | python class body at microbootstrap/settings.py:29 (t=7832, 11 atoms) |
| 2.6 | 1602 | 7832 | +6230 | 1.00 | late | BaseServiceSettings — pydantic SettingsConfigDict | python class body at microbootstrap/settings.py:29 (t=7832, 7 atoms) |
| 2.7 | 1658 | 2941 | +1283 | 1.00 | late | ENV_PREFIX module-level constant | python decl names surface in microbootstrap/settings.py (t=2941, 3 atoms) |
| 2.8 | 1737 | 3190 | +1453 | 1.00 | late | ServerConfig (granian server fields) | python class body at microbootstrap/settings.py:53 (t=3190, 4 atoms) |
| 2.9 | 1999 | 3390 | +1391 | 0.93 | late | LitestarSettings + FastApiSettings — MRO mixins | python decl at microbootstrap/settings.py:60 (t=3283, 12 atoms) |
| 2.10 | 2223 | 3109 | +886 | 0.87 | late | FastStreamSettings + InstrumentsSetupperSettings — MRO mixins | python decl at microbootstrap/settings.py:90 (t=3096, 10 atoms) |
| 2.12 | 2490 | 8423 | +5933 | 1.00 | late | ApplicationBootstrapper — fluent builder method signatures | python method sigs in microbootstrap/bootstrappers/base.py (t=8188, 8 atoms) |
| 2.13 | 2761 | — | — | 0.04 | missing | ApplicationBootstrapper.bootstrap — the orchestration body | python method sigs in microbootstrap/bootstrappers/base.py (t=8188, 2 atoms) |
| 2.14 | 2973 | — | — | 0.63 | partial | ApplicationBootstrapper — overridable hook docstrings | python method sigs in microbootstrap/bootstrappers/base.py (t=8188, 8 atoms) |
| 3.1 | 3469 | — | — | 0.41 | missing | InstrumentBox — initialize / configure_instrument | python method sigs in microbootstrap/instruments/instrument_box.py (t=2176, 8 atoms) |
| 3.2 | 3619 | — | — | 0.42 | missing | LitestarBootstrapper — class declaration + bootstrap_before | python decl at microbootstrap/bootstrappers/litestar.py:48 (t=5741, 3 atoms) |
| 3.3 | 3944 | — | — | 0.20 | missing | FastApiBootstrapper — class declaration + lifespan glue | python decl at microbootstrap/bootstrappers/fastapi.py:27 (t=4548, 3 atoms) |
| 3.4 | 4224 | — | — | 0.19 | missing | FastStreamBootstrapper — class declaration + bootstrap_before | python decl names surface in microbootstrap/bootstrappers/faststream.py (t=5335, 4 atoms) |
| 3.5 | 4439 | 5659 | +1220 | 0.93 | aligned | Litestar instrument subclass locations — class def lines | python decl names surface in microbootstrap/bootstrappers/litestar.py (t=5600, 20 atoms) |
| 3.6 | 4643 | 4523 | -120 | 0.86 | aligned | FastAPI instrument subclass locations — class def lines | python decl names surface in microbootstrap/bootstrappers/fastapi.py (t=4441, 12 atoms) |
| 3.7 | 4803 | 5403 | +600 | 0.80 | aligned | FastStream instrument subclass locations — class def lines | python decl names surface in microbootstrap/bootstrappers/faststream.py (t=5335, 10 atoms) |
| 4.1 | 5145 | — | — | 0.62 | partial | CorsInstrument + CorsConfig (full file) | python class body at microbootstrap/instruments/cors_instrument.py:8 (t=7601, 7 atoms) |
| 4.2 | 5597 | — | — | 0.57 | partial | HealthChecksInstrument + HealthChecksConfig (full file) | python class body at microbootstrap/instruments/health_checks_instrument.py:14 (t=6887, 7 atoms) |
| 4.3 | 5929 | — | — | 0.70 | partial | SwaggerInstrument + SwaggerConfig (full file) | python class body at microbootstrap/instruments/swagger_instrument.py:10 (t=6996, 7 atoms) |
| 4.4 | 6207 | — | — | 0.76 | partial | PyroscopeConfig + PyroscopeInstrument is_ready/teardown | python class body at microbootstrap/instruments/pyroscope_instrument.py:15 (t=9089, 8 atoms) |
| 4.5 | 6520 | — | — | 0.70 | partial | PrometheusConfig variants — Base/Litestar/FastApi/FastStream | python decl names surface in microbootstrap/instruments/prometheus_instrument.py (t=4100, 10 atoms) |
| 4.6 | 6847 | — | — | 0.54 | partial | FastStreamPrometheusMiddlewareProtocol + PrometheusInstrument.is_ready | python decl names surface in microbootstrap/instruments/prometheus_instrument.py (t=4100, 6 atoms) |
| 4.7 | 7114 | — | — | 0.07 | missing | SentryConfig (fields) | python decl names surface in microbootstrap/instruments/sentry_instrument.py (t=6192, 2 atoms) |
| 4.8 | 7193 | — | — | 0.71 | partial | SentryInstrument — class + is_ready | python decl names surface in microbootstrap/instruments/sentry_instrument.py (t=6192, 2 atoms) |
| 4.9 | 7567 | — | — | 0.05 | missing | OpentelemetryConfig (fields) | python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py (t=9357, 2 atoms) |
| 4.10 | 7807 | — | — | 0.19 | missing | FastStreamOpentelemetryConfig + FastStreamTelemetryMiddlewareProtocol | python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py (t=9357, 4 atoms) |
| 4.11 | 8133 | — | — | 0.04 | missing | BaseOpentelemetryInstrument.is_ready + OpentelemetryInstrument.define_exclude_urls | python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py (t=9357, 2 atoms) |
| 4.12 | 8408 | — | — | 0.16 | missing | LoggingConfig (fields) | python decl names surface in microbootstrap/instruments/logging_instrument.py (t=7319, 2 atoms) |
| 5.1 | 8696 | — | — | 0.00 | missing | examples/litestar_app.py — full file |  |
| 5.2 | 8975 | — | — | 0.00 | missing | examples/fastapi_app.py — full file |  |
| 5.3 | 9203 | — | — | 0.00 | missing | examples/faststream_app.py — broker setup |  |
| 5.4 | 9443 | — | — | 0.67 | partial | InstrumentsSetupper — class + setup/teardown + use_instrument registrations | python method sigs in microbootstrap/instruments_setupper.py (t=3854, 16 atoms) |
| 5.5 | 9505 | 928 | -8577 | 1.00 | early | create_granian_server signature | python decl at microbootstrap/granian_server.py:27 (t=928, 5 atoms) |
| 5.6 | 9734 | 2584 | -7150 | 0.94 | early | helpers — public function signatures | python decl names surface in microbootstrap/helpers.py (t=2453, 10 atoms) |
| 5.7 | 9841 | — | — | 0.60 | partial | Exceptions — full file | python decl names surface in microbootstrap/exceptions.py (t=717, 6 atoms) |
| 5.8 | 9984 | — | — | 0.00 | missing | tests/ directory — fs map |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 263 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 242 | 1.00 | 242 | 1219 | headings outline in README.md |
| 201 | 0.94 | 214 | 7319 | python decl names surface in microbootstrap/instruments/logging_instrument.py |
| 195 | 0.73 | 268 | 9357 | python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py |
| 172 | 1.00 | 172 | 9881 | python imports in microbootstrap/instruments_setupper.py |
| 170 | 1.00 | 170 | 6779 | README.md section #1 |
| 166 | 1.00 | 166 | 9709 | python imports in microbootstrap/settings.py |
| 154 | 0.84 | 184 | 6192 | python decl names surface in microbootstrap/instruments/sentry_instrument.py |
| 146 | 0.90 | 163 | 8771 | python method sigs in microbootstrap/instruments/logging_instrument.py |
| 136 | 1.00 | 136 | 1400 | package dependencies in package.json |
| 97 | 1.00 | 97 | 5109 | python imports in microbootstrap/granian_server.py |
| 93 | 1.00 | 93 | 4808 | README.md section #7 |
| 84 | 1.00 | 84 | 4962 | python class body at microbootstrap/config/litestar.py:13 |
| 83 | 1.00 | 83 | 418 | package scripts in package.json |
| 83 | 1.00 | 83 | 8915 | python decl body at microbootstrap/helpers.py:48 |
| 82 | 1.00 | 82 | 4715 | python imports in microbootstrap/helpers.py |
| 81 | 1.00 | 81 | 8527 | python imports in microbootstrap/instruments/base.py |
| 81 | 1.00 | 81 | 8608 | python imports in microbootstrap/middlewares/fastapi.py |
| 79 | 1.00 | 79 | 3718 | python decl at microbootstrap/granian_server.py:16 |
| 77 | 1.00 | 77 | 7985 | python imports in microbootstrap/instruments/prometheus_instrument.py |
| 76 | 1.00 | 76 | 7908 | python imports in microbootstrap/config/litestar.py |
| 72 | 1.00 | 72 | 7068 | python decl doc at microbootstrap/bootstrappers/litestar.py:133 |
| 71 | 0.52 | 136 | 3854 | python method sigs in microbootstrap/instruments_setupper.py |
| 68 | 1.00 | 68 | 104 | package identity in package.json |
| 65 | 0.38 | 172 | 5335 | python decl names surface in microbootstrap/bootstrappers/faststream.py |
| 61 | 1.00 | 61 | 7421 | python decl at microbootstrap/instruments/logging_instrument.py:36 |
| 61 | 1.00 | 61 | 3958 | python method at microbootstrap/instruments_setupper.py:40 |
| 56 | 1.00 | 56 | 3546 | python imports in microbootstrap/console_writer.py |
| 55 | 0.66 | 83 | 2707 | python decl names surface in microbootstrap/bootstrappers/base.py |
| 55 | 0.32 | 172 | 5600 | python decl names surface in microbootstrap/bootstrappers/litestar.py |
| 54 | 1.00 | 54 | 5163 | python imports in microbootstrap/instruments/instrument_box.py |

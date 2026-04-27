scores: Sim=0.342 Reached=5/50 Early=3 Late=2 Partial=0 Missing=45 Used=9085/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 4 | 0 | 5 | 0.43 |
| 2 | 14 | 0 | 0 | 14 | 0.00 |
| 3 | 7 | 0 | 0 | 7 | 0.00 |
| 4 | 12 | 0 | 0 | 12 | 0.00 |
| 5 | 8 | 1 | 0 | 7 | 0.12 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 26 | — | — | 0.00 | missing | README one-liner statement of purpose |  |
| 1.2 | 59 | 33 | -26 | 1.00 | early | Repo top-level layout |  |
| 1.3 | 158 | — | — | 0.00 | missing | README — list of supported instruments |  |
| 1.4 | 223 | — | — | 0.00 | missing | README — list of target frameworks |  |
| 1.5 | 315 | 510 | +195 | 1.00 | late | Package top-level files + small subdirs |  |
| 1.6 | 410 | 1028 | +618 | 1.00 | late | Bootstrappers + instruments subdir contents |  |
| 1.7 | 610 | — | — | 0.00 | missing | Public API re-exports (__all__) |  |
| 1.8 | 795 | — | — | 0.00 | missing | README — canonical Litestar quickstart snippet |  |
| 1.9 | 1027 | 335 | -692 | 0.89 | early | README table-of-contents (H2/H3 outline) | README headline in README.md (t=335, 17 atoms) |
| 2.1 | 1093 | — | — | 0.00 | missing | Instrument ABC — class declaration |  |
| 2.2 | 1126 | — | — | 0.00 | missing | BaseInstrumentConfig (pydantic base) |  |
| 2.3 | 1201 | — | — | 0.00 | missing | Instrument ABC — abstract methods (signatures) |  |
| 2.4 | 1345 | — | — | 0.00 | missing | Instrument ABC — overridable hook signatures |  |
| 2.5 | 1527 | — | — | 0.00 | missing | BaseServiceSettings — service_* fields |  |
| 2.6 | 1602 | — | — | 0.00 | missing | BaseServiceSettings — pydantic SettingsConfigDict |  |
| 2.7 | 1658 | — | — | 0.00 | missing | ENV_PREFIX module-level constant |  |
| 2.8 | 1737 | — | — | 0.00 | missing | ServerConfig (granian server fields) |  |
| 2.9 | 1999 | — | — | 0.00 | missing | LitestarSettings + FastApiSettings — MRO mixins |  |
| 2.10 | 2223 | — | — | 0.00 | missing | FastStreamSettings + InstrumentsSetupperSettings — MRO mixins |  |
| 2.11 | 2296 | — | — | 0.00 | missing | ApplicationBootstrapper — generic class signature |  |
| 2.12 | 2490 | — | — | 0.00 | missing | ApplicationBootstrapper — fluent builder method signatures |  |
| 2.13 | 2761 | — | — | 0.00 | missing | ApplicationBootstrapper.bootstrap — the orchestration body |  |
| 2.14 | 2973 | — | — | 0.00 | missing | ApplicationBootstrapper — overridable hook docstrings |  |
| 3.1 | 3469 | — | — | 0.00 | missing | InstrumentBox — initialize / configure_instrument |  |
| 3.2 | 3619 | — | — | 0.00 | missing | LitestarBootstrapper — class declaration + bootstrap_before |  |
| 3.3 | 3944 | — | — | 0.00 | missing | FastApiBootstrapper — class declaration + lifespan glue |  |
| 3.4 | 4224 | — | — | 0.00 | missing | FastStreamBootstrapper — class declaration + bootstrap_before |  |
| 3.5 | 4439 | — | — | 0.00 | missing | Litestar instrument subclass locations — class def lines |  |
| 3.6 | 4643 | — | — | 0.00 | missing | FastAPI instrument subclass locations — class def lines |  |
| 3.7 | 4803 | — | — | 0.00 | missing | FastStream instrument subclass locations — class def lines |  |
| 4.1 | 5145 | — | — | 0.00 | missing | CorsInstrument + CorsConfig (full file) |  |
| 4.2 | 5597 | — | — | 0.00 | missing | HealthChecksInstrument + HealthChecksConfig (full file) |  |
| 4.3 | 5929 | — | — | 0.00 | missing | SwaggerInstrument + SwaggerConfig (full file) |  |
| 4.4 | 6207 | — | — | 0.00 | missing | PyroscopeConfig + PyroscopeInstrument is_ready/teardown |  |
| 4.5 | 6520 | — | — | 0.00 | missing | PrometheusConfig variants — Base/Litestar/FastApi/FastStream |  |
| 4.6 | 6847 | — | — | 0.00 | missing | FastStreamPrometheusMiddlewareProtocol + PrometheusInstrument.is_ready |  |
| 4.7 | 7114 | — | — | 0.00 | missing | SentryConfig (fields) |  |
| 4.8 | 7193 | — | — | 0.00 | missing | SentryInstrument — class + is_ready |  |
| 4.9 | 7567 | — | — | 0.00 | missing | OpentelemetryConfig (fields) |  |
| 4.10 | 7807 | — | — | 0.00 | missing | FastStreamOpentelemetryConfig + FastStreamTelemetryMiddlewareProtocol |  |
| 4.11 | 8133 | — | — | 0.00 | missing | BaseOpentelemetryInstrument.is_ready + OpentelemetryInstrument.define_exclude_urls |  |
| 4.12 | 8408 | — | — | 0.00 | missing | LoggingConfig (fields) |  |
| 5.1 | 8696 | — | — | 0.00 | missing | examples/litestar_app.py — full file |  |
| 5.2 | 8975 | — | — | 0.00 | missing | examples/fastapi_app.py — full file |  |
| 5.3 | 9203 | — | — | 0.00 | missing | examples/faststream_app.py — broker setup |  |
| 5.4 | 9443 | — | — | 0.00 | missing | InstrumentsSetupper — class + setup/teardown + use_instrument registrations |  |
| 5.5 | 9505 | — | — | 0.00 | missing | create_granian_server signature |  |
| 5.6 | 9734 | — | — | 0.00 | missing | helpers — public function signatures |  |
| 5.7 | 9841 | — | — | 0.00 | missing | Exceptions — full file |  |
| 5.8 | 9984 | 4277 | -5707 | 1.00 | early | tests/ directory — fs map |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 19 | 7737 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1019 | 1.00 | 1019 | 3717 | README.md section #20 |
| 861 | 1.00 | 861 | 9085 | README.md section #9 |
| 727 | 1.00 | 727 | 8224 | README.md section #18 |
| 697 | 1.00 | 697 | 7497 | README.md section #10 |
| 600 | 1.00 | 600 | 6800 | README.md section #8 |
| 423 | 1.00 | 423 | 6200 | README.md section #17 |
| 422 | 1.00 | 422 | 5777 | README.md section #12 |
| 410 | 1.00 | 410 | 5355 | README.md section #14 |
| 342 | 1.00 | 342 | 4619 | README.md section #13 |
| 326 | 1.00 | 326 | 4945 | README.md section #19 |
| 310 | 1.00 | 310 | 1773 | README.md section #2 |
| 302 | 1.00 | 302 | 2075 | README.md section #5 |
| 242 | 1.00 | 242 | 777 | headings outline in README.md |
| 240 | 1.00 | 240 | 3957 | README.md section #4 |
| 233 | 1.00 | 233 | 2698 | README.md section #3 |
| 222 | 1.00 | 222 | 4215 | README.md section #15 |
| 172 | 1.00 | 172 | 1463 | README.md section #6 |
| 170 | 1.00 | 170 | 1291 | README.md section #1 |
| 168 | 1.00 | 168 | 2465 | README.md section #11 |
| 159 | 1.00 | 159 | 2234 | plaintext config .gitignore |
| 136 | 1.00 | 136 | 958 | package dependencies in package.json |
| 93 | 1.00 | 93 | 1121 | README.md section #7 |
| 83 | 1.00 | 83 | 418 | package scripts in package.json |
| 68 | 1.00 | 68 | 104 | package identity in package.json |

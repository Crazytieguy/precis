scores: Score(3000)=0.409 ns_rows≤3K=23/50 (reached=7 partial=0 missing=16)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 102 | 0.592 | 0.451 | 0.517 | 980 |
| 1442 | 148 | 0.556 | 0.425 | 0.486 | 1354 |
| 2080 | 205 | 0.527 | 0.307 | 0.402 | 1990 |
| 3000 | 294 | 0.552 | 0.303 | 0.409 | 3000 |
| 4327 | 394 | 0.553 | 0.312 | 0.415 | 4316 |
| 6240 | 555 | 0.560 | 0.350 | 0.443 | 6151 |
| 9000 | 768 | 0.605 | 0.435 | 0.513 | 8869 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (gap@3k=0.06), 32 wrong-slice/granularity (gap@3k=3.57), 0 no-discovered (gap@3k=0.00)
Secondary intervention: promote predecessors for 1 gated candidate
Top rows: 1.1, 1.3, 1.4, 1.8, 2.9, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 32 | 3.62 | 3.57 | 2.86 | nearby candidates have low exact atom overlap | 1.1, 1.3, 1.4, 1.8, 2.9, ... |
| promote python decl at microbootstrap/instruments/opentelemetry_instrument.py:48 | 1 | 0.03 | 0.03 | 0.03 | 0 files, exact total=20/22 | 4.9 |
| tune ranking for high-overlap unscheduled candidates | 1 | 0.02 | 0.02 | 0.02 | high-overlap candidates not in the schedule by T_max, exact total=12/14 | 4.7 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | value/ranking |
| wrong-slice / granularity | 32 | 31 | 1 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 7 | 7 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 1 | 0.03 | promote predecessor |
| too expensive at final margin | 1 | 0.02 | tune ranking |

Candidate hint kinds: scheduled bbox=30, unscheduled bbox=7, scheduled same-file=4, fs-only=1 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 22 |
| scheduled bbox | missing | high | 5 |
| scheduled bbox | missing | full | 2 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | low | 6 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.7 | 7114 | 0.00 | 0.00 | missing | SentryConfig (fields) | [scheduled bbox exact=2/14] python decl names surface in microbootstrap/instruments/sentry_instrument.py (t=9053, 2 atoms); better unscheduled exact=12/14: python class body at microbootstrap/instruments/sentry_instrument.py:15 (12 atoms, too expensive at final margin) |
| 4.9 | 7567 | 0.00 | 0.00 | missing | OpentelemetryConfig (fields) | [unscheduled bbox exact=20/22] python class body at microbootstrap/instruments/opentelemetry_instrument.py:48 (20 atoms, predecessor not scheduled: python decl at microbootstrap/instruments/opentelemetry_instrument.py:48) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 26 | 0.00 | 0.00 | missing | README one-liner statement of purpose | [scheduled same-file] headings outline in README.md (t=1596, 43 atoms) |
| 1.3 | 158 | 0.00 | 0.00 | missing | README — list of supported instruments | [scheduled same-file] headings outline in README.md (t=1596, 43 atoms) |
| 1.4 | 223 | 0.00 | 0.00 | missing | README — list of target frameworks | [scheduled same-file] headings outline in README.md (t=1596, 43 atoms) |
| 1.8 | 795 | 0.00 | 0.00 | missing | README — canonical Litestar quickstart snippet | [scheduled same-file] headings outline in README.md (t=1596, 43 atoms) |
| 2.3 | 1201 | 0.00 | 0.00 | missing | Instrument ABC — abstract methods (signatures) | [scheduled bbox exact=4/7] python method sigs in microbootstrap/instruments/base.py (t=6412, 4 atoms) |
| 2.4 | 1345 | 0.00 | 0.00 | missing | Instrument ABC — overridable hook signatures | [scheduled bbox exact=8/13] python method sigs in microbootstrap/instruments/base.py (t=6412, 8 atoms) |
| 2.5 | 1527 | 0.00 | 0.00 | missing | BaseServiceSettings — service_* fields | [scheduled bbox exact=11/14] python class body at microbootstrap/settings.py:29 (t=7389, 11 atoms) |
| 2.9 | 1999 | 0.00 | 0.00 | missing | LitestarSettings + FastApiSettings — MRO mixins | [scheduled bbox exact=12/28] python decl at microbootstrap/settings.py:75 (t=4597, 12 atoms) |
| 2.10 | 2223 | 0.00 | 0.00 | missing | FastStreamSettings + InstrumentsSetupperSettings — MRO mixins | [scheduled bbox exact=10/23] python decl at microbootstrap/settings.py:90 (t=4303, 10 atoms) |
| 2.12 | 2490 | 0.00 | 0.00 | missing | ApplicationBootstrapper — fluent builder method signatures | [scheduled bbox exact=8/19] python method sigs in microbootstrap/bootstrappers/base.py (t=8449, 8 atoms) |
| 2.13 | 2761 | 0.00 | 0.00 | missing | ApplicationBootstrapper.bootstrap — the orchestration body | [scheduled bbox exact=2/26] python method sigs in microbootstrap/bootstrappers/base.py (t=8449, 2 atoms); better unscheduled exact=8/26: python method body at microbootstrap/bootstrappers/base.py:72 body 74 (8 atoms, discovered unscheduled) |
| 2.14 | 2973 | 0.00 | 0.00 | missing | ApplicationBootstrapper — overridable hook docstrings | [scheduled bbox exact=8/16] python method sigs in microbootstrap/bootstrappers/base.py (t=8449, 8 atoms) |
| 3.1 | 3469 | 0.38 | 0.40 | missing | InstrumentBox — initialize / configure_instrument | [scheduled bbox exact=8/42] python method sigs in microbootstrap/instruments/instrument_box.py (t=2579, 8 atoms) |
| 3.2 | 3619 | 0.00 | 0.00 | missing | LitestarBootstrapper — class declaration + bootstrap_before | [scheduled bbox exact=3/12] python decl at microbootstrap/bootstrappers/litestar.py:48 (t=7967, 3 atoms); better unscheduled exact=5/12: python method body at microbootstrap/bootstrappers/litestar.py:54 body 55 (5 atoms, predecessor not scheduled: python method at microbootstrap/bootstrappers/litestar.py:54) |
| 3.3 | 3944 | 0.00 | 0.00 | missing | FastApiBootstrapper — class declaration + lifespan glue | [scheduled bbox exact=3/25] python decl at microbootstrap/bootstrappers/fastapi.py:27 (t=5729, 3 atoms); better unscheduled exact=6/25: python method sigs in microbootstrap/bootstrappers/fastapi.py (6 atoms, too expensive at final margin) |
| 3.4 | 4224 | 0.00 | 0.00 | missing | FastStreamBootstrapper — class declaration + bootstrap_before | [scheduled bbox exact=4/21] python decl names surface in microbootstrap/bootstrappers/faststream.py (t=7561, 4 atoms); better unscheduled exact=10/21: python method body at microbootstrap/bootstrappers/faststream.py:41 body 42 (10 atoms, predecessor not scheduled: python method at microbootstrap/bootstrappers/faststream.py:41) |
| 4.1 | 5145 | 0.28 | 0.30 | missing | CorsInstrument + CorsConfig (full file) | [scheduled bbox exact=7/29] python class body at microbootstrap/instruments/cors_instrument.py:8 (t=7158, 7 atoms) |
| 4.2 | 5597 | 0.34 | 0.48 | missing | HealthChecksInstrument + HealthChecksConfig (full file) | [scheduled bbox exact=7/42] python class body at microbootstrap/instruments/health_checks_instrument.py:14 (t=6846, 7 atoms) |
| 4.3 | 5929 | 0.27 | 0.32 | missing | SwaggerInstrument + SwaggerConfig (full file) | [scheduled bbox exact=7/30] python class body at microbootstrap/instruments/swagger_instrument.py:10 (t=6955, 7 atoms) |
| 4.4 | 6207 | 0.34 | 0.35 | missing | PyroscopeConfig + PyroscopeInstrument is_ready/teardown | [scheduled bbox exact=8/21] python class body at microbootstrap/instruments/pyroscope_instrument.py:15 (t=9502, 8 atoms) |
| 4.5 | 6520 | 0.00 | 0.00 | missing | PrometheusConfig variants — Base/Litestar/FastApi/FastStream | [scheduled bbox exact=8/20] python decl names surface in microbootstrap/instruments/prometheus_instrument.py (t=5192, 10 atoms) |
| 4.6 | 6847 | 0.00 | 0.00 | missing | FastStreamPrometheusMiddlewareProtocol + PrometheusInstrument.is_ready | [scheduled bbox exact=6/26] python method at microbootstrap/instruments/prometheus_instrument.py:46 (t=5462, 6 atoms); better unscheduled exact=8/26: python method at microbootstrap/instruments/prometheus_instrument.py:37 (8 atoms, discovered unscheduled) |
| 4.8 | 7193 | 0.00 | 0.00 | missing | SentryInstrument — class + is_ready | [scheduled bbox exact=2/7] python method sigs in microbootstrap/instruments/sentry_instrument.py (t=9153, 2 atoms) |
| 4.10 | 7807 | 0.00 | 0.00 | missing | FastStreamOpentelemetryConfig + FastStreamTelemetryMiddlewareProtocol | [unscheduled bbox exact=6/21] python method at microbootstrap/instruments/opentelemetry_instrument.py:82 (6 atoms, predecessor not scheduled: python method sigs in microbootstrap/instruments/opentelemetry_instrument.py) |
| 4.11 | 8133 | 0.00 | 0.00 | missing | BaseOpentelemetryInstrument.is_ready + OpentelemetryInstrument.define_exclude_urls | [unscheduled bbox exact=8/25] python method sigs in microbootstrap/instruments/opentelemetry_instrument.py (10 atoms, predecessor not scheduled: python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py) |
| 4.12 | 8408 | 0.00 | 0.00 | missing | LoggingConfig (fields) | [unscheduled bbox exact=10/19] python class body at microbootstrap/instruments/logging_instrument.py:127 (10 atoms, predecessor not scheduled: python decl at microbootstrap/instruments/logging_instrument.py:127) |
| 5.1 | 8696 | 0.00 | 0.00 | missing | examples/litestar_app.py — full file | [unscheduled bbox exact=7/29] python decl names surface in examples/litestar_app.py (7 atoms, discovered unscheduled) |
| 5.2 | 8975 | 0.00 | 0.00 | missing | examples/fastapi_app.py — full file | [unscheduled bbox exact=7/30] python imports in examples/fastapi_app.py (7 atoms, discovered unscheduled) |
| 5.3 | 9203 | 0.00 | 0.00 | missing | examples/faststream_app.py — broker setup | [unscheduled bbox exact=5/22] python decl body at examples/faststream_app.py:23 body 26 (5 atoms, predecessor not scheduled: python decl at examples/faststream_app.py:23) |
| 5.4 | 9443 | 0.05 | 0.03 | missing | InstrumentsSetupper — class + setup/teardown + use_instrument registrations | [scheduled bbox exact=8/21] python method sigs in microbootstrap/instruments_setupper.py (t=3864, 16 atoms) |
| 5.6 | 9734 | 0.00 | 0.00 | missing | helpers — public function signatures | [scheduled bbox exact=9/16] python decl names surface in microbootstrap/helpers.py (t=3387, 10 atoms) |
| 5.7 | 9841 | 0.60 | 0.99 | partial | Exceptions — full file | [scheduled bbox exact=6/10] python decl names surface in microbootstrap/exceptions.py (t=877, 6 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.8 | 9984 | 0.00 | 0.00 | missing | tests/ directory — fs map | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.6 | 1602 | 0.00 | 0.00 | missing | BaseServiceSettings — pydantic SettingsConfigDict | [scheduled bbox exact=7/7] python class body at microbootstrap/settings.py:29 (t=7389, 7 atoms) |
| 2.7 | 1658 | 0.00 | 0.00 | missing | ENV_PREFIX module-level constant | [scheduled bbox exact=3/3] python decl names surface in microbootstrap/settings.py (t=4148, 3 atoms) |
| 2.8 | 1737 | 0.00 | 0.00 | missing | ServerConfig (granian server fields) | [scheduled bbox exact=4/5] python class body at microbootstrap/settings.py:53 (t=4397, 4 atoms) |
| 2.11 | 2296 | 0.00 | 0.00 | missing | ApplicationBootstrapper — generic class signature | [scheduled bbox exact=4/5] python class body at microbootstrap/bootstrappers/base.py:25 (t=3166, 4 atoms) |
| 3.5 | 4439 | 0.00 | 0.00 | missing | Litestar instrument subclass locations — class def lines | [scheduled bbox exact=14/15] python decl names surface in microbootstrap/bootstrappers/litestar.py (t=7826, 20 atoms) |
| 3.6 | 4643 | 0.00 | 0.00 | missing | FastAPI instrument subclass locations — class def lines | [scheduled bbox exact=12/14] python decl names surface in microbootstrap/bootstrappers/fastapi.py (t=5622, 12 atoms) |
| 3.7 | 4803 | 0.00 | 0.00 | missing | FastStream instrument subclass locations — class def lines | [scheduled bbox exact=8/10] python decl names surface in microbootstrap/bootstrappers/faststream.py (t=7561, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 263 | README.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 277 | 0.58 | 477 | 2467 | python imports in microbootstrap/__init__.py |
| 242 | 1.00 | 242 | 1596 | headings outline in README.md |
| 136 | 1.00 | 136 | 1810 | package dependencies in package.json |
| 83 | 1.00 | 83 | 808 | package scripts in package.json |
| 68 | 1.00 | 68 | 104 | package identity in package.json |
| 56 | 1.00 | 56 | 3000 | python imports in microbootstrap/console_writer.py |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 172 | 1.00 | 172 | 9886 | python imports in microbootstrap/instruments_setupper.py |
| 170 | 1.00 | 170 | 6738 | README.md section #1 |
| 166 | 1.00 | 166 | 9714 | python imports in microbootstrap/settings.py |
| 154 | 0.84 | 184 | 9053 | python decl names surface in microbootstrap/instruments/sentry_instrument.py |
| 97 | 1.00 | 97 | 5882 | python imports in microbootstrap/granian_server.py |
| 93 | 1.00 | 93 | 4958 | README.md section #7 |
| 92 | 1.00 | 92 | 4859 | python method sigs #1 in microbootstrap/instruments/opentelemetry_instrument.py |
| 84 | 1.00 | 84 | 5071 | python class body at microbootstrap/config/litestar.py:13 |
| 83 | 1.00 | 83 | 9328 | python decl body at microbootstrap/helpers.py:48 body 52 |
| 82 | 1.00 | 82 | 4767 | python imports in microbootstrap/helpers.py |
| 827 | — | — | — | +12 more rows |

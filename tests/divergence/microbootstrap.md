scores: Sim=0.381 Reached=21/50 Early=4 Late=15 Partial=11 Missing=18 Used=9957/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (w×gap=0.05), 23 wrong-slice/granularity (w×gap=4.50), 3 no-discovered (w×gap=0.03)
Secondary intervention: free final budget for 1 too-expensive candidate
Loss reasons: 1 predecessor-gated, 1 too-expensive, 0 discovered-unscheduled
Top rows: 1.1, 1.3, 1.4, 1.8, 2.13, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 23 | 4.50 | 7/14/23 | nearby candidates have low exact atom overlap | 1.1, 1.3, 1.4, 1.8, 2.13, ... |
| add walker candidates for no-discovered rows | 3 | 0.03 | 0/0/3 | NS rows have no discovered line candidate | 5.1, 5.2, 5.3 |
| free final budget / demote late waste | 1 | 0.03 | 0/0/1 | high-overlap candidates exceed final remaining budget, exact total=12/14 | 4.7 |
| promote python decl at microbootstrap/instruments/opentelemetry_instrument.py:48 | 1 | 0.02 | 0/0/1 | 0 files, exact total=20/22 | 4.9 |

Tiers: 1=5/9 reached, 0 partial, 4 missing, avg=0.54; 2=11/14 reached, 2 partial, 1 missing, avg=0.86; 3=3/7 reached, 0 partial, 4 missing, avg=0.54; 4=0/12 reached, 7 partial, 5 missing, avg=0.40; 5=2/8 reached, 2 partial, 4 missing, avg=0.40

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 23 | 12 | 11 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 21 | 0 | 0 | 21 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 1 | 0.02 | promote predecessor |
| too expensive at final margin | 1 | 0.03 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=36, unscheduled bbox=3, scheduled same-file=4, fs-only=4, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 6 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 4 |
| scheduled bbox | missing | low | 7 |
| scheduled bbox | partial | low | 11 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.7 | 7114 | — | — | 0.07 | missing | SentryConfig (fields) | [scheduled bbox exact=2/14] python decl names surface in microbootstrap/instruments/sentry_instrument.py (t=7006, 2 atoms); better unscheduled exact=12/14: python class body at microbootstrap/instruments/sentry_instrument.py:15 (12 atoms, too expensive at final margin) |
| 4.9 | 7567 | — | — | 0.00 | missing | OpentelemetryConfig (fields) | [unscheduled bbox exact=20/22] python class body at microbootstrap/instruments/opentelemetry_instrument.py:48 (20 atoms, predecessor not scheduled: python decl at microbootstrap/instruments/opentelemetry_instrument.py:48) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 26 | — | — | 0.00 | missing | README one-liner statement of purpose | [scheduled same-file] headings outline in README.md (t=1252, 43 atoms) |
| 1.3 | 158 | — | — | 0.00 | missing | README — list of supported instruments | [scheduled same-file] headings outline in README.md (t=1252, 43 atoms) |
| 1.4 | 223 | — | — | 0.00 | missing | README — list of target frameworks | [scheduled same-file] headings outline in README.md (t=1252, 43 atoms) |
| 1.8 | 795 | — | — | 0.00 | missing | README — canonical Litestar quickstart snippet | [scheduled same-file] headings outline in README.md (t=1252, 43 atoms) |
| 2.4 | 1345 | — | — | 0.77 | partial | Instrument ABC — overridable hook signatures | [scheduled bbox exact=8/13] python method sigs in microbootstrap/instruments/base.py (t=7275, 8 atoms) |
| 2.13 | 2761 | — | — | 0.04 | missing | ApplicationBootstrapper.bootstrap — the orchestration body | [scheduled bbox exact=2/26] python method sigs in microbootstrap/bootstrappers/base.py (t=9010, 2 atoms); better unscheduled exact=8/26: python method body at microbootstrap/bootstrappers/base.py:72 body 74 (8 atoms, too expensive at final margin) |
| 2.14 | 2973 | — | — | 0.63 | partial | ApplicationBootstrapper — overridable hook docstrings | [scheduled bbox exact=8/16] python method sigs in microbootstrap/bootstrappers/base.py (t=9010, 8 atoms) |
| 3.1 | 3469 | — | — | 0.41 | missing | InstrumentBox — initialize / configure_instrument | [scheduled bbox exact=8/42] python method sigs in microbootstrap/instruments/instrument_box.py (t=2701, 8 atoms) |
| 3.2 | 3619 | — | — | 0.42 | missing | LitestarBootstrapper — class declaration + bootstrap_before | [scheduled bbox exact=3/12] python decl at microbootstrap/bootstrappers/litestar.py:48 (t=6519, 3 atoms); better unscheduled exact=5/12: python method body at microbootstrap/bootstrappers/litestar.py:54 body 55 (5 atoms, predecessor not scheduled: python method at microbootstrap/bootstrappers/litestar.py:54) |
| 3.3 | 3944 | — | — | 0.20 | missing | FastApiBootstrapper — class declaration + lifespan glue | [scheduled bbox exact=3/25] python decl at microbootstrap/bootstrappers/fastapi.py:27 (t=5117, 3 atoms); better unscheduled exact=6/25: python method sigs in microbootstrap/bootstrappers/fastapi.py (6 atoms, too expensive at final margin) |
| 3.4 | 4224 | — | — | 0.19 | missing | FastStreamBootstrapper — class declaration + bootstrap_before | [scheduled bbox exact=4/21] python decl names surface in microbootstrap/bootstrappers/faststream.py (t=6113, 4 atoms); better unscheduled exact=10/21: python method body at microbootstrap/bootstrappers/faststream.py:41 body 42 (10 atoms, predecessor not scheduled: python method at microbootstrap/bootstrappers/faststream.py:41) |
| 4.1 | 5145 | — | — | 0.62 | partial | CorsInstrument + CorsConfig (full file) | [scheduled bbox exact=7/29] python class body at microbootstrap/instruments/cors_instrument.py:8 (t=8423, 7 atoms) |
| 4.2 | 5597 | — | — | 0.57 | partial | HealthChecksInstrument + HealthChecksConfig (full file) | [scheduled bbox exact=7/42] python class body at microbootstrap/instruments/health_checks_instrument.py:14 (t=7709, 7 atoms) |
| 4.3 | 5929 | — | — | 0.70 | partial | SwaggerInstrument + SwaggerConfig (full file) | [scheduled bbox exact=7/30] python class body at microbootstrap/instruments/swagger_instrument.py:10 (t=7818, 7 atoms) |
| 4.4 | 6207 | — | — | 0.76 | partial | PyroscopeConfig + PyroscopeInstrument is_ready/teardown | [scheduled bbox exact=8/21] python class body at microbootstrap/instruments/pyroscope_instrument.py:15 (t=9957, 8 atoms) |
| 4.5 | 6520 | — | — | 0.70 | partial | PrometheusConfig variants — Base/Litestar/FastApi/FastStream | [scheduled bbox exact=8/20] python decl names surface in microbootstrap/instruments/prometheus_instrument.py (t=4669, 10 atoms) |
| 4.6 | 6847 | — | — | 0.54 | partial | FastStreamPrometheusMiddlewareProtocol + PrometheusInstrument.is_ready | [scheduled bbox exact=6/26] python method at microbootstrap/instruments/prometheus_instrument.py:46 (t=5541, 6 atoms); better unscheduled exact=8/26: python method at microbootstrap/instruments/prometheus_instrument.py:37 (8 atoms, too expensive at final margin) |
| 4.8 | 7193 | — | — | 0.71 | partial | SentryInstrument — class + is_ready | [scheduled bbox exact=2/7] python method sigs in microbootstrap/instruments/sentry_instrument.py (t=7106, 2 atoms) |
| 4.10 | 7807 | — | — | 0.00 | missing | FastStreamOpentelemetryConfig + FastStreamTelemetryMiddlewareProtocol | [unscheduled bbox exact=6/21] python method at microbootstrap/instruments/opentelemetry_instrument.py:82 (6 atoms, predecessor not scheduled: python method sigs in microbootstrap/instruments/opentelemetry_instrument.py) |
| 4.11 | 8133 | — | — | 0.00 | missing | BaseOpentelemetryInstrument.is_ready + OpentelemetryInstrument.define_exclude_urls | [unscheduled bbox exact=8/25] python method sigs in microbootstrap/instruments/opentelemetry_instrument.py (10 atoms, predecessor not scheduled: python decl names surface in microbootstrap/instruments/opentelemetry_instrument.py) |
| 4.12 | 8408 | — | — | 0.16 | missing | LoggingConfig (fields) | [scheduled bbox exact=2/19] python method at microbootstrap/instruments/logging_instrument.py:140 (t=9683, 2 atoms); better unscheduled exact=10/19: python class body at microbootstrap/instruments/logging_instrument.py:127 (10 atoms, too expensive at final margin) |
| 5.4 | 9443 | — | — | 0.67 | partial | InstrumentsSetupper — class + setup/teardown + use_instrument registrations | [scheduled bbox exact=8/21] python method sigs in microbootstrap/instruments_setupper.py (t=4423, 16 atoms) |
| 5.7 | 9841 | — | — | 0.60 | partial | Exceptions — full file | [scheduled bbox exact=6/10] python decl names surface in microbootstrap/exceptions.py (t=717, 6 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.1 | 8696 | — | — | 0.00 | missing | examples/litestar_app.py — full file | no discovered line candidate |
| 5.2 | 8975 | — | — | 0.00 | missing | examples/fastapi_app.py — full file | no discovered line candidate |
| 5.3 | 9203 | — | — | 0.00 | missing | examples/faststream_app.py — broker setup | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.8 | 9984 | — | — | 0.00 | missing | tests/ directory — fs map | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 59 | 33 | -26 | 1.00 | early | Repo top-level layout | fs-only |
| 1.5 | 315 | 613 | +298 | 1.00 | late | Package top-level files + small subdirs | fs-only |
| 1.6 | 410 | 2043 | +1633 | 1.00 | late | Bootstrappers + instruments subdir contents | fs-only |
| 1.7 | 610 | 1925 | +1315 | 1.00 | late | Public API re-exports (__all__) | [scheduled bbox exact=19/19] python imports in microbootstrap/__init__.py (t=1925, 19 atoms) |
| 1.9 | 1027 | 335 | -692 | 0.89 | early | README table-of-contents (H2/H3 outline) | [scheduled bbox exact=17/19] README headline in README.md (t=335, 17 atoms) |
| 2.1 | 1093 | 2637 | +1544 | 1.00 | late | Instrument ABC — class declaration | [scheduled bbox exact=3/5] python class body at microbootstrap/instruments/base.py:23 (t=2637, 3 atoms) |
| 2.2 | 1126 | 2599 | +1473 | 1.00 | late | BaseInstrumentConfig (pydantic base) | [scheduled bbox exact=2/2] python decl names surface in microbootstrap/instruments/base.py (t=2566, 2 atoms) |
| 2.3 | 1201 | 7410 | +6209 | 0.86 | late | Instrument ABC — abstract methods (signatures) | [scheduled bbox exact=4/7] python method sigs in microbootstrap/instruments/base.py (t=7275, 4 atoms) |
| 2.5 | 1527 | 8654 | +7127 | 1.00 | late | BaseServiceSettings — service_* fields | [scheduled bbox exact=11/14] python class body at microbootstrap/settings.py:29 (t=8654, 11 atoms) |
| 2.6 | 1602 | 8654 | +7052 | 1.00 | late | BaseServiceSettings — pydantic SettingsConfigDict | [scheduled bbox exact=7/7] python class body at microbootstrap/settings.py:29 (t=8654, 7 atoms) |
| 2.7 | 1658 | 3466 | +1808 | 1.00 | late | ENV_PREFIX module-level constant | [scheduled bbox exact=3/3] python decl names surface in microbootstrap/settings.py (t=3466, 3 atoms) |
| 2.8 | 1737 | 3715 | +1978 | 1.00 | late | ServerConfig (granian server fields) | [scheduled bbox exact=4/5] python class body at microbootstrap/settings.py:53 (t=3715, 4 atoms) |
| 2.9 | 1999 | 3915 | +1916 | 0.93 | late | LitestarSettings + FastApiSettings — MRO mixins | [scheduled bbox exact=12/28] python decl at microbootstrap/settings.py:75 (t=3915, 12 atoms) |
| 2.10 | 2223 | 3634 | +1411 | 0.87 | late | FastStreamSettings + InstrumentsSetupperSettings — MRO mixins | [scheduled bbox exact=10/23] python decl at microbootstrap/settings.py:90 (t=3621, 10 atoms) |
| 2.11 | 2296 | 3295 | +999 | 1.00 | late | ApplicationBootstrapper — generic class signature | [scheduled bbox exact=4/5] python class body at microbootstrap/bootstrappers/base.py:25 (t=3295, 4 atoms) |
| 2.12 | 2490 | 9245 | +6755 | 1.00 | late | ApplicationBootstrapper — fluent builder method signatures | [scheduled bbox exact=8/19] python method sigs in microbootstrap/bootstrappers/base.py (t=9010, 8 atoms) |
| 3.5 | 4439 | 6437 | +1998 | 0.93 | late | Litestar instrument subclass locations — class def lines | [scheduled bbox exact=14/15] python decl names surface in microbootstrap/bootstrappers/litestar.py (t=6378, 20 atoms) |
| 3.6 | 4643 | 5092 | +449 | 0.86 | aligned | FastAPI instrument subclass locations — class def lines | [scheduled bbox exact=12/14] python decl names surface in microbootstrap/bootstrappers/fastapi.py (t=5010, 12 atoms) |
| 3.7 | 4803 | 6181 | +1378 | 0.80 | aligned | FastStream instrument subclass locations — class def lines | [scheduled bbox exact=8/10] python decl names surface in microbootstrap/bootstrappers/faststream.py (t=6113, 10 atoms) |
| 5.5 | 9505 | 961 | -8544 | 1.00 | early | create_granian_server signature | [scheduled bbox exact=5/5] python decl at microbootstrap/granian_server.py:27 (t=961, 5 atoms) |
| 5.6 | 9734 | 3109 | -6625 | 0.94 | early | helpers — public function signatures | [scheduled bbox exact=9/16] python decl names surface in microbootstrap/helpers.py (t=2978, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 263 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 277 | 0.58 | 477 | 1925 | python imports in microbootstrap/__init__.py |
| 242 | 1.00 | 242 | 1252 | headings outline in README.md |
| 201 | 0.94 | 214 | 8141 | python decl names surface in microbootstrap/instruments/logging_instrument.py |
| 170 | 1.00 | 170 | 7601 | README.md section #1 |
| 154 | 0.84 | 184 | 7006 | python decl names surface in microbootstrap/instruments/sentry_instrument.py |
| 146 | 0.90 | 163 | 9639 | python method sigs in microbootstrap/instruments/logging_instrument.py |
| 136 | 1.00 | 136 | 1448 | package dependencies in package.json |
| 97 | 1.00 | 97 | 5819 | python imports in microbootstrap/granian_server.py |
| 93 | 1.00 | 93 | 5489 | README.md section #7 |
| 92 | 1.00 | 92 | 5390 | python method sigs #1 in microbootstrap/instruments/opentelemetry_instrument.py |
| 1344 | — | — | — | +19 more rows |

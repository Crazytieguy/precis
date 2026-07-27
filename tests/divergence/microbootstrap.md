Score(3000)=0.553 I=0.788 C=0.388 ns_rows≤3K=15/45 (reached=7 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | listing of '.' |  |  | 1.000 |
| ns | 33 |  | 33 | Fixture root listing | 1.1 |  | 1.000 |
| walker |  | 36 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 45 | 9 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 88 |  | 55 | microbootstrap/ package listing | 1.2 |  | 0.622 |
| walker |  | 100 | 55 | listing of 'microbootstrap' |  |  | 1.000 |
| walker |  | 119 | 19 | python imports #1 in microbootstrap/__init__.py |  |  | 1.000 |
| ns | 134 |  | 46 | microbootstrap/{bootstrappers,config}/ listings | 1.3 |  | 0.813 |
| walker |  | 137 | 18 | python imports #2 in microbootstrap/__init__.py |  |  | 0.813 |
| walker |  | 153 | 16 | python imports #3 in microbootstrap/__init__.py |  |  | 0.814 |
| walker |  | 173 | 20 | python imports #6 in microbootstrap/__init__.py |  |  | 0.814 |
| walker |  | 191 | 18 | python imports #7 in microbootstrap/__init__.py |  |  | 0.814 |
| ns | 220 |  | 86 | microbootstrap/{instruments,middlewares}/ listings | 1.4 |  | 0.651 |
| ns | 363 |  | 143 | tests/ tree listing | 1.5 |  | 0.509 |
| walker |  | 391 | 200 | python imports in microbootstrap/__init__.py |  |  | 0.515 |
| walker |  | 407 | 16 | python imports #8 in microbootstrap/__init__.py |  |  | 0.516 |
| walker |  | 423 | 16 | listing of 'microbootstrap/middlewares' |  |  | 0.524 |
| walker |  | 480 | 57 | python imports #4 in microbootstrap/__init__.py |  |  | 0.525 |
| walker |  | 501 | 21 | listing of 'microbootstrap/config' |  |  | 0.549 |
| walker |  | 526 | 25 | listing of 'microbootstrap/bootstrappers' |  |  | 0.642 |
| ns | 554 |  | 191 | README: supported instruments + frameworks catalog | 1.7 |  | 0.559 |
| walker |  | 589 | 63 | python imports #5 in microbootstrap/__init__.py |  |  | 0.562 |
| walker |  | 639 | 50 | python imports #9 in microbootstrap/__init__.py |  |  | 0.565 |
| walker |  | 709 | 70 | listing of 'microbootstrap/instruments' |  |  | 0.697 |
| walker |  | 743 | 34 | manifest config in pyproject.toml |  |  | 0.697 |
| walker |  | 756 | 13 | python decl names surface in microbootstrap/instruments_setupper.py |  |  | 0.697 |
| walker |  | 756 | 0 | python decl at microbootstrap/instruments_setupper.py:19 |  |  | 0.697 |
| walker |  | 770 | 14 | python decl names surface in microbootstrap/console_writer.py |  |  | 0.606 |
| ns | 770 |  | 216 | README lede + minimal usage sketch | 1.8 |  | 0.606 |
| walker |  | 778 | 8 | python decl at microbootstrap/console_writer.py:10 |  |  | 0.606 |
| walker |  | 796 | 18 | listing of 'examples' |  |  | 0.606 |
| ns | 806 |  | 36 | pyproject.toml: project identity | 1.9 |  | 0.596 |
| walker |  | 867 | 71 | declaration surface of Justfile |  |  | 0.596 |
| walker |  | 1126 | 259 | README headline in README.md |  |  | 0.598 |
| ns | 1143 |  | 337 | instruments/base.py: BaseInstrumentConfig + Instrument fields | 2.1 |  | 0.514 |
| walker |  | 1148 | 22 | python class body at microbootstrap/instruments_setupper.py:19 |  |  | 0.514 |
| walker |  | 1177 | 29 | python decl names surface in microbootstrap/granian_server.py |  |  | 0.514 |
| walker |  | 1261 | 84 | tool.coverage+pytest config in pyproject.toml |  |  | 0.514 |
| walker |  | 1308 | 47 | python method sigs in microbootstrap/console_writer.py |  |  | 0.514 |
| walker |  | 1308 | 0 | python method at microbootstrap/console_writer.py:16 |  |  | 0.514 |
| walker |  | 1308 | 0 | python method at microbootstrap/console_writer.py:31 |  |  | 0.514 |
| walker |  | 1353 | 45 | listing of 'tests' |  |  | 0.532 |
| walker |  | 1367 | 14 | python decl names surface in microbootstrap/config/fastapi.py |  |  | 0.532 |
| walker |  | 1376 | 9 | python decl at microbootstrap/config/fastapi.py:20 |  |  | 0.532 |
| walker |  | 1390 | 14 | python decl names surface in microbootstrap/config/faststream.py |  |  | 0.532 |
| walker |  | 1399 | 9 | python decl at microbootstrap/config/faststream.py:20 |  |  | 0.532 |
| walker |  | 1413 | 14 | python decl names surface in microbootstrap/config/litestar.py |  |  | 0.532 |
| walker |  | 1424 | 11 | python decl at microbootstrap/config/litestar.py:13 |  |  | 0.532 |
| walker |  | 1438 | 14 | python decl names surface in microbootstrap/instruments/instrument_box.py |  |  | 0.532 |
| ns | 1445 |  | 302 | instruments/base.py: Instrument lifecycle methods | 2.2 |  | 0.480 |
| walker |  | 1446 | 8 | python decl at microbootstrap/instruments/instrument_box.py:9 |  |  | 0.480 |
| walker |  | 1555 | 109 | tool.uv+mypy config in pyproject.toml |  |  | 0.480 |
| ns | 1724 |  | 279 | microbootstrap/__init__.py: imports | 2.3 |  | 0.541 |
| walker |  | 1795 | 240 | headings outline in README.md |  |  | 0.541 |
| walker |  | 1847 | 52 | python decl at microbootstrap/granian_server.py:27 |  |  | 0.541 |
| walker |  | 1863 | 16 | python decl names surface in microbootstrap/middlewares/fastapi.py |  |  | 0.541 |
| walker |  | 1889 | 26 | python decl at microbootstrap/middlewares/fastapi.py:12 |  |  | 0.541 |
| ns | 1922 |  | 198 | microbootstrap/__init__.py: __all__ surface | 2.4 |  | 0.577 |
| walker |  | 1934 | 45 | python decl names surface in microbootstrap/exceptions.py |  |  | 0.577 |
| walker |  | 1934 | 0 | python decl at microbootstrap/exceptions.py:1 |  |  | 0.577 |
| walker |  | 1934 | 0 | python decl at microbootstrap/exceptions.py:5 |  |  | 0.577 |
| walker |  | 1934 | 0 | python decl at microbootstrap/exceptions.py:9 |  |  | 0.577 |
| walker |  | 1943 | 9 | python decl doc at microbootstrap/exceptions.py:1 |  |  | 0.577 |
| walker |  | 1959 | 16 | python decl doc at microbootstrap/exceptions.py:5 |  |  | 0.577 |
| walker |  | 1976 | 17 | python decl doc at microbootstrap/exceptions.py:9 |  |  | 0.577 |
| walker |  | 2023 | 47 | python class body at microbootstrap/console_writer.py:10 |  |  | 0.577 |
| walker |  | 2040 | 17 | python decl names surface in microbootstrap/middlewares/litestar.py |  |  | 0.577 |
| walker |  | 2065 | 25 | python decl at microbootstrap/middlewares/litestar.py:14 |  |  | 0.577 |
| walker |  | 2080 | 15 | README.md section #22 |  |  | 0.577 |
| walker |  | 2150 | 70 | [package] in pyproject.toml |  |  | 0.588 |
| ns | 2197 |  | 275 | bootstrappers/base.py: ApplicationBootstrapper.bootstrap() | 2.5 |  | 0.551 |
| walker |  | 2203 | 53 | README.md section #33 |  |  | 0.551 |
| walker |  | 2253 | 50 | python method at microbootstrap/console_writer.py:22 |  |  | 0.551 |
| walker |  | 2289 | 36 | listing of 'tests/bootstrappers' |  |  | 0.569 |
| ns | 2421 |  | 224 | instruments/instrument_box.py: fields + initialize() | 2.6 |  | 0.545 |
| walker |  | 2425 | 136 | python method sigs in microbootstrap/instruments_setupper.py |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:23 |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:28 |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:51 |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:57 |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:62 |  |  | 0.545 |
| walker |  | 2425 | 0 | python method at microbootstrap/instruments_setupper.py:65 |  |  | 0.545 |
| walker |  | 2431 | 6 | python method body at microbootstrap/instruments_setupper.py:62 body 63 |  |  | 0.545 |
| walker |  | 2440 | 9 | python method body at microbootstrap/instruments_setupper.py:65 body 66 |  |  | 0.545 |
| walker |  | 2472 | 32 | python method at microbootstrap/instruments_setupper.py:32 |  |  | 0.545 |
| walker |  | 2553 | 81 | python decl names surface in microbootstrap/instruments/base.py |  |  | 0.550 |
| walker |  | 2553 | 0 | python decl at microbootstrap/instruments/base.py:19 |  |  | 0.550 |
| walker |  | 2571 | 18 | python decl at microbootstrap/instruments/base.py:23 |  |  | 0.552 |
| walker |  | 2588 | 17 | python class body at microbootstrap/instruments/base.py:19 |  |  | 0.555 |
| walker |  | 2725 | 137 | python method sigs in microbootstrap/instruments/base.py |  |  | 0.566 |
| walker |  | 2725 | 0 | python method at microbootstrap/instruments/base.py:35 |  |  | 0.566 |
| walker |  | 2725 | 0 | python method at microbootstrap/instruments/base.py:50 |  |  | 0.566 |
| walker |  | 2725 | 0 | python method at microbootstrap/instruments/base.py:53 |  |  | 0.566 |
| walker |  | 2725 | 0 | python method at microbootstrap/instruments/base.py:56 |  |  | 0.566 |
| walker |  | 2725 | 0 | python method at microbootstrap/instruments/base.py:60 |  |  | 0.566 |
| walker |  | 2735 | 10 | python method at microbootstrap/instruments/base.py:42 |  |  | 0.568 |
| walker |  | 2735 | 0 | python method body at microbootstrap/instruments/base.py:42 body 43 |  |  | 0.568 |
| walker |  | 2751 | 16 | python method at microbootstrap/instruments/base.py:45 |  |  | 0.572 |
| ns | 2757 |  | 336 | instruments/instrument_box.py: configure_instrument() + extend_instruments() | 2.7 |  | 0.537 |
| walker |  | 2779 | 28 | python method at microbootstrap/instruments/base.py:29 |  |  | 0.543 |
| walker |  | 2815 | 36 | python class body at microbootstrap/instruments/base.py:23 |  |  | 0.553 |
| walker |  | 3028 | 213 | README.md section #1 |  |  | 0.553 |
| walker |  | 3058 | 30 | python decl names surface in microbootstrap/instruments/cors_instrument.py |  |  | 0.553 |
| walker |  | 3058 | 0 | python decl at microbootstrap/instruments/cors_instrument.py:8 |  |  | 0.553 |
| walker |  | 3058 | 0 | python decl at microbootstrap/instruments/cors_instrument.py:18 |  |  | 0.553 |
| walker |  | 3092 | 34 | python method sigs in microbootstrap/instruments/cors_instrument.py |  |  | 0.553 |
| walker |  | 3092 | 0 | python method at microbootstrap/instruments/cors_instrument.py:22 |  |  | 0.553 |
| walker |  | 3100 | 8 | python method at microbootstrap/instruments/cors_instrument.py:27 |  |  | 0.553 |
| walker |  | 3107 | 7 | python method body at microbootstrap/instruments/cors_instrument.py:27 body 29 |  |  | 0.553 |
| ns | 3112 |  | 355 | settings.py: BaseServiceSettings + ServerConfig | 2.8 |  | 0.524 |
| walker |  | 3137 | 30 | python decl names surface in microbootstrap/instruments/swagger_instrument.py |  |  | 0.524 |
| walker |  | 3137 | 0 | python decl at microbootstrap/instruments/swagger_instrument.py:10 |  |  | 0.524 |
| walker |  | 3137 | 0 | python decl at microbootstrap/instruments/swagger_instrument.py:21 |  |  | 0.524 |
| walker |  | 3171 | 34 | python method sigs in microbootstrap/instruments/swagger_instrument.py |  |  | 0.524 |
| walker |  | 3171 | 0 | python method at microbootstrap/instruments/swagger_instrument.py:25 |  |  | 0.524 |
| walker |  | 3179 | 8 | python method at microbootstrap/instruments/swagger_instrument.py:28 |  |  | 0.524 |
| walker |  | 3186 | 7 | python method body at microbootstrap/instruments/swagger_instrument.py:28 body 30 |  |  | 0.524 |
| walker |  | 3248 | 62 | python method sigs in microbootstrap/instruments/instrument_box.py |  |  | 0.526 |
| walker |  | 3248 | 0 | python method at microbootstrap/instruments/instrument_box.py:14 |  |  | 0.526 |
| walker |  | 3256 | 8 | python method at microbootstrap/instruments/instrument_box.py:48 |  |  | 0.526 |
| walker |  | 3266 | 10 | python method body at microbootstrap/instruments/instrument_box.py:48 body 50 |  |  | 0.527 |
| walker |  | 3361 | 95 | python decl names surface in microbootstrap/bootstrappers/base.py |  |  | 0.527 |
| walker |  | 3361 | 0 | python decl at microbootstrap/bootstrappers/base.py:17 |  |  | 0.527 |
| walker |  | 3361 | 0 | python decl at microbootstrap/bootstrappers/base.py:25 |  |  | 0.527 |
| ns | 3374 |  | 262 | settings.py: LitestarSettings + FastApiSettings | 2.9 |  | 0.502 |
| walker |  | 3379 | 18 | python class body at microbootstrap/bootstrappers/base.py:17 |  |  | 0.502 |
| walker |  | 3542 | 163 | python method sigs in microbootstrap/bootstrappers/base.py |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:31 |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:72 |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:99 |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:103 |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:107 |  |  | 0.502 |
| walker |  | 3542 | 0 | python method at microbootstrap/bootstrappers/base.py:111 |  |  | 0.502 |
| walker |  | 3549 | 7 | python method body at microbootstrap/bootstrappers/base.py:99 body 101 |  |  | 0.502 |
| walker |  | 3557 | 8 | python method body at microbootstrap/bootstrappers/base.py:103 body 105 |  |  | 0.502 |
| walker |  | 3588 | 31 | python method at microbootstrap/bootstrappers/base.py:39 |  |  | 0.502 |
| ns | 3596 |  | 222 | settings.py: FastStreamSettings + InstrumentsSetupperSettings | 2.10 |  | 0.484 |
| walker |  | 3619 | 31 | python method at microbootstrap/bootstrappers/base.py:46 |  |  | 0.484 |
| walker |  | 3651 | 32 | python method at microbootstrap/bootstrappers/base.py:53 |  |  | 0.484 |
| walker |  | 3696 | 45 | python class body at microbootstrap/bootstrappers/base.py:25 |  |  | 0.484 |
| walker |  | 3713 | 17 | python method doc at microbootstrap/bootstrappers/base.py:99 |  |  | 0.484 |
| walker |  | 3774 | 61 | python method at microbootstrap/instruments_setupper.py:40 |  |  | 0.484 |
| walker |  | 3807 | 33 | python decl names surface in microbootstrap/instruments/pyroscope_instrument.py |  |  | 0.484 |
| walker |  | 3807 | 0 | python decl at microbootstrap/instruments/pyroscope_instrument.py:15 |  |  | 0.484 |
| walker |  | 3807 | 0 | python decl at microbootstrap/instruments/pyroscope_instrument.py:27 |  |  | 0.484 |
| walker |  | 3870 | 63 | python method sigs in microbootstrap/instruments/pyroscope_instrument.py |  |  | 0.484 |
| walker |  | 3870 | 0 | python method at microbootstrap/instruments/pyroscope_instrument.py:31 |  |  | 0.484 |
| walker |  | 3870 | 0 | python method at microbootstrap/instruments/pyroscope_instrument.py:34 |  |  | 0.484 |
| walker |  | 3870 | 0 | python method at microbootstrap/instruments/pyroscope_instrument.py:37 |  |  | 0.484 |
| walker |  | 3877 | 7 | python method body at microbootstrap/instruments/pyroscope_instrument.py:34 body 35 |  |  | 0.484 |
| walker |  | 3885 | 8 | python method at microbootstrap/instruments/pyroscope_instrument.py:52 |  |  | 0.484 |
| walker |  | 3893 | 8 | python method body at microbootstrap/instruments/pyroscope_instrument.py:52 body 54 |  |  | 0.484 |
| walker |  | 3912 | 19 | python method doc at microbootstrap/bootstrappers/base.py:107 |  |  | 0.484 |
| walker |  | 3931 | 19 | python method doc at microbootstrap/instruments/base.py:56 |  |  | 0.488 |
| ns | 3938 |  | 342 | cors_instrument.py (full) | 3.1 |  | 0.470 |
| walker |  | 3950 | 19 | python method doc at microbootstrap/instruments/base.py:60 |  |  | 0.475 |
| walker |  | 4029 | 79 | python decl at microbootstrap/granian_server.py:16 |  |  | 0.475 |
| ns | 4270 |  | 332 | swagger_instrument.py (full) | 3.2 |  | 0.458 |
| walker |  | 4336 | 307 | tool.ruff config in pyproject.toml |  |  | 0.458 |
| walker |  | 4499 | 163 | python decl names surface in microbootstrap/settings.py |  |  | 0.460 |
| walker |  | 4499 | 0 | python decl at microbootstrap/settings.py:53 |  |  | 0.460 |
| walker |  | 4516 | 17 | python decl at microbootstrap/settings.py:29 |  |  | 0.460 |
| walker |  | 4567 | 51 | python decl at microbootstrap/settings.py:105 |  |  | 0.466 |
| walker |  | 4577 | 10 | python decl doc at microbootstrap/settings.py:105 |  |  | 0.468 |
| walker |  | 4658 | 81 | python decl at microbootstrap/settings.py:90 |  |  | 0.489 |
| walker |  | 4675 | 17 | python class body at microbootstrap/settings.py:90 |  |  | 0.494 |
| walker |  | 4686 | 11 | python decl doc at microbootstrap/settings.py:90 |  |  | 0.499 |
| ns | 4722 |  | 452 | health_checks_instrument.py (full) | 3.3 |  | 0.473 |
| walker |  | 4750 | 64 | python class body at microbootstrap/settings.py:53 |  |  | 0.476 |
| walker |  | 4845 | 95 | python decl at microbootstrap/settings.py:60 |  |  | 0.487 |
| walker |  | 4857 | 12 | python decl doc at microbootstrap/settings.py:60 |  |  | 0.490 |
| walker |  | 4952 | 95 | python decl at microbootstrap/settings.py:75 |  |  | 0.516 |
| walker |  | 4964 | 12 | python decl doc at microbootstrap/settings.py:75 |  |  | 0.520 |
| walker |  | 5198 | 234 | python class body at microbootstrap/settings.py:29 |  |  | 0.562 |
| walker |  | 5227 | 29 | README.md section #42 |  |  | 0.562 |
| walker |  | 5289 | 62 | listing of 'tests/instruments' |  |  | 0.599 |
| ns | 5353 |  | 631 | pyroscope_instrument.py (full) | 3.4 |  | 0.565 |
| walker |  | 5451 | 162 | python decl names surface in microbootstrap/helpers.py |  |  | 0.565 |
| walker |  | 5451 | 0 | python decl at microbootstrap/helpers.py:19 |  |  | 0.565 |
| walker |  | 5451 | 0 | python decl at microbootstrap/helpers.py:96 |  |  | 0.565 |
| walker |  | 5490 | 39 | python decl at microbootstrap/helpers.py:48 |  |  | 0.565 |
| walker |  | 5530 | 40 | python decl at microbootstrap/helpers.py:60 |  |  | 0.565 |
| walker |  | 5546 | 16 | python decl body at microbootstrap/helpers.py:96 body 97 |  |  | 0.565 |
| walker |  | 5588 | 42 | python decl at microbootstrap/helpers.py:35 |  |  | 0.565 |
| walker |  | 5630 | 42 | python decl at microbootstrap/helpers.py:100 |  |  | 0.565 |
| walker |  | 5712 | 82 | python class body at microbootstrap/config/litestar.py:13 |  |  | 0.565 |
| walker |  | 5746 | 34 | README.md section #15 |  |  | 0.565 |
| ns | 5758 |  | 405 | sentry_instrument.py: SentryConfig | 3.5 |  | 0.548 |
| walker |  | 5852 | 106 | README.md section #7 |  |  | 0.548 |
| walker |  | 5881 | 29 | python method doc at microbootstrap/bootstrappers/base.py:103 |  |  | 0.548 |
| walker |  | 5917 | 36 | README.md section #18 |  |  | 0.548 |
| walker |  | 5970 | 53 | python decl names surface in microbootstrap/instruments/health_checks_instrument.py |  |  | 0.548 |
| walker |  | 5970 | 0 | python decl at microbootstrap/instruments/health_checks_instrument.py:8 |  |  | 0.548 |
| walker |  | 5970 | 0 | python decl at microbootstrap/instruments/health_checks_instrument.py:14 |  |  | 0.548 |
| walker |  | 5970 | 0 | python decl at microbootstrap/instruments/health_checks_instrument.py:26 |  |  | 0.548 |
| walker |  | 6025 | 55 | python method sigs in microbootstrap/instruments/health_checks_instrument.py |  |  | 0.550 |
| walker |  | 6025 | 0 | python method at microbootstrap/instruments/health_checks_instrument.py:30 |  |  | 0.550 |
| walker |  | 6025 | 0 | python method at microbootstrap/instruments/health_checks_instrument.py:37 |  |  | 0.550 |
| walker |  | 6033 | 8 | python method at microbootstrap/instruments/health_checks_instrument.py:40 |  |  | 0.550 |
| walker |  | 6041 | 8 | python method body at microbootstrap/instruments/health_checks_instrument.py:40 body 42 |  |  | 0.551 |
| walker |  | 6052 | 11 | python method body at microbootstrap/instruments/health_checks_instrument.py:37 body 38 |  |  | 0.552 |
| walker |  | 6090 | 38 | README.md section #9 |  |  | 0.552 |
| walker |  | 6140 | 50 | python imports in microbootstrap/helpers.py |  |  | 0.552 |
| ns | 6161 |  | 403 | sentry_instrument.py: is_ready + bootstrap() + before_send helper locations | 3.6 |  | 0.535 |
| walker |  | 6166 | 26 | python imports in microbootstrap/config/faststream.py |  |  | 0.535 |
| ns | 6413 |  | 252 | prometheus_instrument.py: Base/Litestar/FastApi configs | 3.7 |  | 0.527 |
| ns | 6505 |  | 92 | prometheus_instrument.py: FastStream protocol location + config | 3.8 |  | 0.523 |
| walker |  | 6862 | 696 | [dependencies] in pyproject.toml |  |  | 0.523 |
| ns | 6888 |  | 383 | opentelemetry_instrument.py: OpentelemetryConfig | 3.9 |  | 0.512 |
| walker |  | 6923 | 61 | python method at microbootstrap/bootstrappers/base.py:61 |  |  | 0.512 |
| walker |  | 7110 | 187 | python class body at microbootstrap/config/faststream.py:20 |  |  | 0.512 |
| ns | 7145 |  | 257 | opentelemetry_instrument.py: bootstrap() resource + tracer_provider | 3.10 |  | 0.504 |
| ns | 7383 |  | 238 | opentelemetry_instrument.py: bootstrap() exporters + instrumentors | 3.11 |  | 0.496 |
| walker |  | 7475 | 365 | README.md section #5 |  |  | 0.496 |
| walker |  | 7498 | 23 | python class body at microbootstrap/instruments/swagger_instrument.py:21 |  |  | 0.497 |
| walker |  | 7557 | 59 | python imports in microbootstrap/console_writer.py |  |  | 0.497 |
| walker |  | 7581 | 24 | python class body at microbootstrap/instruments/cors_instrument.py:18 |  |  | 0.499 |
| walker |  | 7605 | 24 | python class body at microbootstrap/instruments/pyroscope_instrument.py:27 |  |  | 0.500 |
| walker |  | 7610 | 5 | python method body at microbootstrap/instruments/base.py:56 body 58 |  |  | 0.501 |
| ns | 7662 |  | 279 | logging_instrument.py: LoggingConfig | 3.12 |  | 0.493 |
| walker |  | 7671 | 61 | python imports in microbootstrap/granian_server.py |  |  | 0.493 |
| walker |  | 7717 | 46 | README.md section #21 |  |  | 0.493 |
| walker |  | 7743 | 26 | python class body at microbootstrap/instruments/health_checks_instrument.py:26 |  |  | 0.495 |
| walker |  | 7752 | 9 | python decl body at microbootstrap/helpers.py:60 body 93 |  |  | 0.495 |
| walker |  | 7784 | 32 | python class body at microbootstrap/instruments/health_checks_instrument.py:8 |  |  | 0.497 |
| walker |  | 7828 | 44 | python imports in microbootstrap/instruments/health_checks_instrument.py |  |  | 0.503 |
| walker |  | 7856 | 28 | python method at microbootstrap/instruments/instrument_box.py:21 |  |  | 0.504 |
| ns | 7884 |  | 222 | logging_instrument.py: _configure_structlog_loggers (debug vs prod) | 3.13 |  | 0.495 |
| walker |  | 7901 | 45 | python imports in microbootstrap/instruments/cors_instrument.py |  |  | 0.499 |
| walker |  | 7922 | 21 | python method body at microbootstrap/instruments/swagger_instrument.py:25 body 26 |  |  | 0.501 |
| walker |  | 7938 | 16 | python method body at microbootstrap/instruments/pyroscope_instrument.py:31 body 32 |  |  | 0.502 |
| ns | 8056 |  | 172 | litestar.py: Bootstrapper + instrument-class roster | 4.1 |  | 0.498 |
| walker |  | 8112 | 174 | python decl names surface in microbootstrap/bootstrappers/litestar.py |  |  | 0.498 |
| walker |  | 8112 | 0 | python decl at microbootstrap/bootstrappers/litestar.py:127 |  |  | 0.498 |
| walker |  | 8112 | 0 | python decl at microbootstrap/bootstrappers/litestar.py:158 |  |  | 0.498 |
| walker |  | 8125 | 13 | python decl at microbootstrap/bootstrappers/litestar.py:108 |  |  | 0.499 |
| walker |  | 8138 | 13 | python decl at microbootstrap/bootstrappers/litestar.py:190 |  |  | 0.499 |
| walker |  | 8152 | 14 | python decl at microbootstrap/bootstrappers/litestar.py:62 |  |  | 0.500 |
| walker |  | 8166 | 14 | python decl at microbootstrap/bootstrappers/litestar.py:73 |  |  | 0.501 |
| walker |  | 8181 | 15 | python decl at microbootstrap/bootstrappers/litestar.py:222 |  |  | 0.503 |
| walker |  | 8199 | 18 | python decl at microbootstrap/bootstrappers/litestar.py:177 |  |  | 0.504 |
| ns | 8212 |  | 156 | fastapi.py: Bootstrapper + instrument-class roster | 4.2 |  | 0.501 |
| walker |  | 8220 | 21 | python decl at microbootstrap/bootstrappers/litestar.py:199 |  |  | 0.503 |
| walker |  | 8243 | 23 | python decl at microbootstrap/bootstrappers/litestar.py:133 |  |  | 0.503 |
| walker |  | 8271 | 28 | python decl at microbootstrap/bootstrappers/litestar.py:48 |  |  | 0.503 |
| ns | 8379 |  | 167 | faststream.py: Bootstrapper + instrument-class roster | 4.3 |  | 0.500 |
| ns | 8426 |  | 47 | bootstrappers/base.py: ApplicationBootstrapper.configure_application/configure_instrument(s)/use_instrument locations | 5.1 |  | 0.503 |
| walker |  | 8479 | 208 | python method sigs in microbootstrap/bootstrappers/litestar.py |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:54 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:64 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:75 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:110 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:159 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:179 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:192 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:201 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:224 |  |  | 0.503 |
| walker |  | 8479 | 0 | python method at microbootstrap/bootstrappers/litestar.py:236 |  |  | 0.503 |
| walker |  | 8487 | 8 | python method at microbootstrap/bootstrappers/litestar.py:217 |  |  | 0.503 |
| walker |  | 8511 | 24 | python class body at microbootstrap/bootstrappers/litestar.py:48 |  |  | 0.503 |
| ns | 8574 |  | 148 | instruments_setupper.py: InstrumentsSetupper fields + __init__ + configure_instrument(s)/use_instrument locations | 5.2 |  | 0.503 |
| walker |  | 8598 | 87 | python decl doc at microbootstrap/bootstrappers/litestar.py:133 |  |  | 0.503 |
| walker |  | 8619 | 21 | python method body at microbootstrap/instruments_setupper.py:28 body 29 |  |  | 0.503 |
| ns | 8747 |  | 173 | instruments_setupper.py: setup()/teardown() + instrument registrations | 5.3 |  | 0.498 |
| walker |  | 8824 | 205 | README.md section #6 |  |  | 0.498 |
| walker |  | 8830 | 6 | python method body at microbootstrap/instruments/base.py:50 body 51 |  |  | 0.499 |
| walker |  | 8890 | 60 | README.md section #39 |  |  | 0.499 |
| walker |  | 9025 | 135 | python decl names surface in microbootstrap/bootstrappers/fastapi.py |  |  | 0.499 |
| walker |  | 9038 | 13 | python decl at microbootstrap/bootstrappers/fastapi.py:73 |  |  | 0.499 |
| walker |  | 9051 | 13 | python decl at microbootstrap/bootstrappers/fastapi.py:103 |  |  | 0.500 |
| walker |  | 9065 | 14 | python decl at microbootstrap/bootstrappers/fastapi.py:57 |  |  | 0.500 |
| walker |  | 9080 | 15 | python decl at microbootstrap/bootstrappers/fastapi.py:136 |  |  | 0.502 |
| walker |  | 9098 | 18 | python decl at microbootstrap/bootstrappers/fastapi.py:92 |  |  | 0.503 |
| walker |  | 9119 | 21 | python decl at microbootstrap/bootstrappers/fastapi.py:113 |  |  | 0.505 |
| walker |  | 9146 | 27 | python decl at microbootstrap/bootstrappers/fastapi.py:27 |  |  | 0.505 |
| ns | 9186 |  | 439 | helpers.py: merge_dict_configs | 5.4 |  | 0.493 |
| ns | 9293 |  | 107 | exceptions.py (full) | 5.5 |  | 0.495 |
| ns | 9365 |  | 72 | config/ + middlewares/ locations | 5.6 |  | 0.498 |
| ns | 9400 |  | 35 | console_writer.py locations | 5.7 |  | 0.500 |
| walker |  | 9403 | 257 | python method sigs in microbootstrap/bootstrappers/fastapi.py |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:47 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:59 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:67 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:75 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:94 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:105 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:115 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:138 |  |  | 0.500 |
| walker |  | 9403 | 0 | python method at microbootstrap/bootstrappers/fastapi.py:150 |  |  | 0.500 |
| walker |  | 9411 | 8 | python method at microbootstrap/bootstrappers/fastapi.py:131 |  |  | 0.500 |
| walker |  | 9436 | 25 | python class body at microbootstrap/bootstrappers/fastapi.py:27 |  |  | 0.500 |
| walker |  | 9446 | 10 | python method at microbootstrap/bootstrappers/fastapi.py:33 |  |  | 0.500 |
| walker |  | 9458 | 12 | python method at microbootstrap/bootstrappers/fastapi.py:41 |  |  | 0.500 |
| walker |  | 9512 | 54 | python imports in microbootstrap/instruments/pyroscope_instrument.py |  |  | 0.503 |
| ns | 9567 |  | 167 | tests/conftest.py: fixture roster (session/app + first half of instrument configs) | 9.1 |  | 0.500 |
| ns | 9750 |  | 183 | tests/conftest.py: fixture roster (remaining configs + settings/console-writer + generic mocks/autouse) | 9.2 |  | 0.496 |
| ns | 9867 |  | 117 | Justfile: core recipes | 10.1 |  | 0.493 |
| ns | 9940 |  | 73 | README: simple vs complex type override semantics | 11.1 |  | 0.492 |

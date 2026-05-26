Score(3000)=0.607 I=0.855 C=0.431 ns_rows≤3K=21/53 (reached=12 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 54 | 4 | listing of 'src' |  |  | 1.000 |
| ns | 67 |  | 17 | README lede — title | 1.2 |  | 0.959 |
| walker |  | 71 | 17 | README headline in README.md |  |  | 1.000 |
| ns | 95 |  | 28 | Version + package name | 1.3 |  | 0.927 |
| walker |  | 118 | 47 | listing of 'src/chronos' |  |  | 0.943 |
| ns | 142 |  | 47 | src/chronos directory listing | 1.4 |  | 0.948 |
| ns | 172 |  | 30 | src/chronos/chronos2 directory listing | 1.5 |  | 0.821 |
| ns | 262 |  | 90 | Package public API — first half of __init__ re-exports | 1.6 |  | 0.711 |
| ns | 310 |  | 48 | Package public API — second half of __init__ re-exports | 1.7 |  | 0.691 |
| walker |  | 441 | 323 | python imports in src/chronos/__init__.py |  |  | 0.859 |
| walker |  | 471 | 30 | listing of 'src/chronos/chronos2' |  |  | 0.965 |
| ns | 490 |  | 180 | README — introduction sentence + Chronos-2 paragraph | 1.8 |  | 0.905 |
| walker |  | 498 | 27 | listing of 'notebooks' |  |  | 0.905 |
| walker |  | 597 | 99 | headings outline in README.md |  |  | 0.920 |
| walker |  | 614 | 17 | README.md section #12 |  |  | 0.920 |
| ns | 737 |  | 247 | README — Chronos-Bolt and Chronos paragraphs | 1.9 |  | 0.898 |
| walker |  | 769 | 155 | python imports in src/chronos/chronos2/__init__.py |  |  | 0.898 |
| walker |  | 796 | 27 | README.md section #11 |  |  | 0.898 |
| ns | 809 |  | 72 | README H2/H3 headings catalog | 1.10 |  | 0.897 |
| ns | 842 |  | 33 | ForecastType enum | 2.1 |  | 0.871 |
| walker |  | 902 | 106 | [dependencies] in pyproject.toml |  |  | 0.871 |
| walker |  | 964 | 62 | [package] in pyproject.toml |  |  | 0.804 |
| ns | 964 |  | 122 | BaseChronosPipeline — class + properties | 2.2 |  | 0.804 |
| walker |  | 1004 | 40 | python decl names surface in src/chronos/base.py |  |  | 0.809 |
| walker |  | 1004 | 0 | python decl at src/chronos/base.py:27 |  |  | 0.809 |
| walker |  | 1004 | 0 | python decl at src/chronos/base.py:32 |  |  | 0.809 |
| walker |  | 1004 | 0 | python decl at src/chronos/base.py:44 |  |  | 0.809 |
| walker |  | 1019 | 15 | python class body at src/chronos/base.py:32 |  |  | 0.809 |
| walker |  | 1041 | 22 | python class body at src/chronos/base.py:27 |  |  | 0.833 |
| walker |  | 1076 | 35 | python class body at src/chronos/base.py:44 |  |  | 0.844 |
| walker |  | 1092 | 16 | listing of 'scripts' |  |  | 0.844 |
| ns | 1124 |  | 160 | BaseChronosPipeline — predict / predict_quantiles signatures | 2.3 |  | 0.793 |
| walker |  | 1258 | 166 | python method sigs in src/chronos/base.py |  |  | 0.811 |
| walker |  | 1258 | 0 | python method at src/chronos/base.py:35 |  |  | 0.811 |
| walker |  | 1258 | 0 | python method at src/chronos/base.py:48 |  |  | 0.811 |
| walker |  | 1258 | 0 | python method at src/chronos/base.py:66 |  |  | 0.811 |
| walker |  | 1258 | 0 | python method at src/chronos/base.py:76 |  |  | 0.811 |
| walker |  | 1270 | 12 | python method at src/chronos/base.py:58 |  |  | 0.821 |
| walker |  | 1282 | 12 | python method at src/chronos/base.py:62 |  |  | 0.834 |
| ns | 1316 |  | 192 | BaseChronosPipeline.predict_df — signature | 2.4 |  | 0.763 |
| walker |  | 1321 | 39 | python method at src/chronos/base.py:252 |  |  | 0.765 |
| walker |  | 1386 | 65 | python method at src/chronos/base.py:336 |  |  | 0.766 |
| walker |  | 1396 | 10 | python method body at src/chronos/base.py:58 body 60 |  |  | 0.778 |
| walker |  | 1406 | 10 | python method body at src/chronos/base.py:62 body 64 |  |  | 0.791 |
| walker |  | 1416 | 10 | python method body at src/chronos/base.py:76 body 98 |  |  | 0.791 |
| walker |  | 1432 | 16 | python method doc at src/chronos/base.py:35 |  |  | 0.792 |
| walker |  | 1447 | 15 | python imports in src/chronos/__about__.py |  |  | 0.808 |
| ns | 1705 |  | 389 | BaseChronosPipeline.from_pretrained — signature + dispatch | 2.5 |  | 0.694 |
| walker |  | 1912 | 465 | README.md section #0 |  |  | 0.694 |
| walker |  | 1990 | 78 | listing of 'test' |  |  | 0.696 |
| walker |  | 2029 | 39 | README.md section #5 |  |  | 0.696 |
| walker |  | 2056 | 27 | python decl names surface in src/chronos/chronos2/config.py |  |  | 0.696 |
| walker |  | 2056 | 0 | python decl at src/chronos/chronos2/config.py:12 |  |  | 0.696 |
| walker |  | 2066 | 10 | python decl at src/chronos/chronos2/config.py:102 |  |  | 0.696 |
| walker |  | 2087 | 21 | python method sigs in src/chronos/chronos2/config.py |  |  | 0.696 |
| ns | 2091 |  | 386 | README — predict_df example call | 2.6 |  | 0.621 |
| walker |  | 2099 | 12 | python method at src/chronos/chronos2/config.py:114 |  |  | 0.621 |
| walker |  | 2127 | 28 | python decl names surface in src/chronos/chronos2/pipeline.py |  |  | 0.621 |
| walker |  | 2127 | 0 | python decl at src/chronos/chronos2/pipeline.py:39 |  |  | 0.621 |
| ns | 2148 |  | 57 | BaseChronosPipeline.predict_fev — signature | 2.7 |  | 0.624 |
| walker |  | 2158 | 31 | python class body at src/chronos/chronos2/pipeline.py:39 |  |  | 0.625 |
| ns | 2273 |  | 125 | PipelineRegistry metaclass | 2.8 |  | 0.611 |
| walker |  | 2276 | 118 | python method at src/chronos/base.py:100 |  |  | 0.647 |
| walker |  | 2286 | 10 | python method body at src/chronos/base.py:100 body 133 |  |  | 0.647 |
| walker |  | 2319 | 33 | README.md section #9 |  |  | 0.647 |
| walker |  | 2322 | 3 | listing of 'ci' |  |  | 0.648 |
| walker |  | 2330 | 8 | README headline in scripts/README.md |  |  | 0.648 |
| walker |  | 2346 | 16 | python method body at src/chronos/chronos2/config.py:114 body 119 |  |  | 0.648 |
| walker |  | 2391 | 45 | python decl names surface in src/chronos/chronos2/trainer.py |  |  | 0.648 |
| walker |  | 2391 | 0 | python decl at src/chronos/chronos2/trainer.py:17 |  |  | 0.648 |
| walker |  | 2391 | 0 | python decl at src/chronos/chronos2/trainer.py:30 |  |  | 0.648 |
| walker |  | 2391 | 0 | python decl at src/chronos/chronos2/trainer.py:40 |  |  | 0.648 |
| walker |  | 2407 | 16 | python decl doc at src/chronos/chronos2/trainer.py:30 |  |  | 0.648 |
| walker |  | 2476 | 69 | python method sigs in src/chronos/chronos2/trainer.py |  |  | 0.648 |
| walker |  | 2476 | 0 | python method at src/chronos/chronos2/trainer.py:33 |  |  | 0.648 |
| walker |  | 2476 | 0 | python method at src/chronos/chronos2/trainer.py:46 |  |  | 0.648 |
| walker |  | 2476 | 0 | python method at src/chronos/chronos2/trainer.py:77 |  |  | 0.648 |
| walker |  | 2532 | 56 | python decl doc at src/chronos/chronos2/trainer.py:40 |  |  | 0.649 |
| walker |  | 2560 | 28 | python method doc at src/chronos/chronos2/config.py:114 |  |  | 0.650 |
| ns | 2574 |  | 301 | ChronosPipeline + ChronosBoltPipeline + Chronos2Pipeline — class declarations | 3.1 |  | 0.603 |
| walker |  | 2741 | 181 | python method at src/chronos/base.py:135 |  |  | 0.660 |
| ns | 2797 |  | 223 | ChronosConfig dataclass — all fields | 3.2 |  | 0.620 |
| walker |  | 2808 | 67 | README.md section #3 |  |  | 0.620 |
| ns | 2882 |  | 85 | ChronosBoltConfig dataclass — all fields | 3.3 |  | 0.606 |
| walker |  | 2890 | 82 | python class body at src/chronos/chronos2/config.py:12 |  |  | 0.607 |
| walker |  | 2965 | 75 | README.md section #1 |  |  | 0.607 |
| walker |  | 3024 | 59 | python method doc at src/chronos/base.py:48 |  |  | 0.607 |
| ns | 3085 |  | 203 | Chronos2ForecastingConfig dataclass — all fields | 3.4 |  | 0.591 |
| walker |  | 3088 | 64 | python method doc at src/chronos/base.py:336 |  |  | 0.591 |
| walker |  | 3200 | 112 | python class body at src/chronos/chronos2/config.py:102 |  |  | 0.634 |
| walker |  | 3260 | 60 | python imports in src/chronos/utils.py |  |  | 0.634 |
| walker |  | 3336 | 76 | python decl names surface in src/chronos/chronos2/model.py |  |  | 0.634 |
| walker |  | 3336 | 0 | python decl at src/chronos/chronos2/model.py:38 |  |  | 0.634 |
| walker |  | 3336 | 0 | python decl at src/chronos/chronos2/model.py:89 |  |  | 0.634 |
| walker |  | 3336 | 0 | python decl at src/chronos/chronos2/model.py:198 |  |  | 0.634 |
| walker |  | 3346 | 10 | python decl at src/chronos/chronos2/model.py:190 |  |  | 0.634 |
| walker |  | 3357 | 11 | python decl at src/chronos/chronos2/model.py:82 |  |  | 0.634 |
| walker |  | 3369 | 12 | python decl at src/chronos/chronos2/model.py:31 |  |  | 0.634 |
| ns | 3413 |  | 328 | Chronos2CoreConfig — class signature + defaults | 3.5 |  | 0.604 |
| walker |  | 3420 | 51 | python class body at src/chronos/chronos2/model.py:31 |  |  | 0.604 |
| ns | 3466 |  | 53 | ChronosTokenizer + MeanScaleUniformBins — class lines | 3.6 |  | 0.600 |
| walker |  | 3484 | 64 | python class body at src/chronos/chronos2/model.py:82 |  |  | 0.600 |
| walker |  | 3548 | 64 | python class body at src/chronos/chronos2/model.py:198 |  |  | 0.601 |
| ns | 3550 |  | 84 | ChronosTokenizer — all abstract method signatures | 3.7 |  | 0.593 |
| walker |  | 3626 | 78 | python class body at src/chronos/chronos2/model.py:190 |  |  | 0.593 |
| walker |  | 3633 | 7 | listing of '.github' |  |  | 0.593 |
| walker |  | 3650 | 17 | listing of '.github/workflows' |  |  | 0.593 |
| walker |  | 3746 | 96 | README.md section #4 |  |  | 0.593 |
| ns | 3807 |  | 257 | ChronosPipeline.predict + predict_quantiles signatures | 3.8 |  | 0.572 |
| ns | 3916 |  | 109 | ChronosBoltPipeline — predict + quantiles property | 3.9 |  | 0.562 |
| walker |  | 3932 | 186 | python method sigs in src/chronos/chronos2/model.py |  |  | 0.562 |
| walker |  | 3932 | 0 | python method at src/chronos/chronos2/model.py:39 |  |  | 0.562 |
| walker |  | 3932 | 0 | python method at src/chronos/chronos2/model.py:90 |  |  | 0.562 |
| walker |  | 3932 | 0 | python method at src/chronos/chronos2/model.py:204 |  |  | 0.562 |
| walker |  | 3932 | 0 | python method at src/chronos/chronos2/model.py:266 |  |  | 0.562 |
| walker |  | 4022 | 90 | python method at src/chronos/chronos2/model.py:134 |  |  | 0.562 |
| walker |  | 4137 | 115 | README.md section #2 |  |  | 0.562 |
| ns | 4139 |  | 223 | Chronos2Pipeline — properties (context, patch, prediction, quantiles) | 3.10 |  | 0.542 |
| walker |  | 4230 | 93 | python method at src/chronos/chronos2/model.py:48 |  |  | 0.542 |
| walker |  | 4264 | 34 | python imports in test/__init__.py |  |  | 0.542 |
| ns | 4296 |  | 157 | Chronos2Pipeline.predict — signature + cross_learning | 3.11 |  | 0.530 |
| walker |  | 4306 | 42 | python decl names surface in src/chronos/df_utils.py |  |  | 0.530 |
| walker |  | 4346 | 40 | python method at src/chronos/chronos2/model.py:98 |  |  | 0.530 |
| walker |  | 4389 | 43 | python method at src/chronos/chronos2/model.py:373 |  |  | 0.530 |
| walker |  | 4437 | 48 | python decl names surface in src/chronos/utils.py |  |  | 0.530 |
| walker |  | 4437 | 0 | python decl at src/chronos/utils.py:11 |  |  | 0.530 |
| walker |  | 4482 | 45 | python decl at src/chronos/utils.py:135 |  |  | 0.530 |
| walker |  | 4534 | 52 | python decl at src/chronos/utils.py:22 |  |  | 0.530 |
| ns | 4543 |  | 247 | Chronos2Pipeline.predict_df — signature (covariates-aware) | 3.12 |  | 0.516 |
| walker |  | 4634 | 100 | python imports in src/chronos/df_utils.py |  |  | 0.516 |
| walker |  | 4759 | 125 | python decl names surface in src/chronos/chronos2/layers.py |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:18 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:86 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:110 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:126 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:148 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:294 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:317 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:343 |  |  | 0.516 |
| walker |  | 4759 | 0 | python decl at src/chronos/chronos2/layers.py:369 |  |  | 0.516 |
| walker |  | 4767 | 8 | python decl at src/chronos/chronos2/layers.py:142 |  |  | 0.517 |
| walker |  | 4776 | 9 | python decl doc at src/chronos/chronos2/layers.py:148 |  |  | 0.517 |
| walker |  | 4795 | 19 | python decl doc at src/chronos/chronos2/layers.py:343 |  |  | 0.517 |
| walker |  | 4814 | 19 | python decl doc at src/chronos/chronos2/layers.py:369 |  |  | 0.517 |
| walker |  | 4845 | 31 | python class body at src/chronos/chronos2/layers.py:142 |  |  | 0.517 |
| ns | 4908 |  | 365 | Chronos2Pipeline.fit — signature (LoRA + validation) | 3.13 |  | 0.497 |
| walker |  | 4930 | 85 | python decl doc at src/chronos/chronos2/layers.py:18 |  |  | 0.497 |
| walker |  | 4942 | 12 | python method body at src/chronos/base.py:48 body 56 |  |  | 0.497 |
| walker |  | 5056 | 114 | python decl body at src/chronos/utils.py:11 body 12 |  |  | 0.497 |
| ns | 5154 |  | 246 | Chronos2Pipeline.from_pretrained + save_pretrained | 3.14 |  | 0.486 |
| ns | 5311 |  | 157 | Chronos2 layers — names of every nn.Module | 4.1 |  | 0.504 |
| walker |  | 5337 | 281 | python method sigs in src/chronos/chronos2/pipeline.py |  |  | 0.507 |
| walker |  | 5337 | 0 | python method at src/chronos/chronos2/pipeline.py:43 |  |  | 0.507 |
| walker |  | 5337 | 0 | python method at src/chronos/chronos2/pipeline.py:1224 |  |  | 0.507 |
| walker |  | 5349 | 12 | python method at src/chronos/chronos2/pipeline.py:76 |  |  | 0.508 |
| walker |  | 5361 | 12 | python method at src/chronos/chronos2/pipeline.py:84 |  |  | 0.509 |
| walker |  | 5374 | 13 | python method at src/chronos/chronos2/pipeline.py:80 |  |  | 0.511 |
| walker |  | 5387 | 13 | python method at src/chronos/chronos2/pipeline.py:88 |  |  | 0.513 |
| walker |  | 5400 | 13 | python method at src/chronos/chronos2/pipeline.py:92 |  |  | 0.515 |
| walker |  | 5422 | 22 | python method at src/chronos/chronos2/pipeline.py:1185 |  |  | 0.516 |
| walker |  | 5437 | 15 | python method body at src/chronos/chronos2/pipeline.py:76 body 78 |  |  | 0.519 |
| walker |  | 5452 | 15 | python method body at src/chronos/chronos2/pipeline.py:88 body 90 |  |  | 0.522 |
| walker |  | 5468 | 16 | python method body at src/chronos/chronos2/pipeline.py:80 body 82 |  |  | 0.526 |
| walker |  | 5485 | 17 | python method body at src/chronos/chronos2/pipeline.py:92 body 94 |  |  | 0.531 |
| walker |  | 5518 | 33 | python method doc at src/chronos/chronos2/pipeline.py:1224 |  |  | 0.534 |
| walker |  | 5538 | 20 | python method body at src/chronos/chronos2/pipeline.py:1224 body 1228 |  |  | 0.536 |
| ns | 5553 |  | 242 | Chronos2 encoder block + encoder — class lines | 4.2 |  | 0.523 |
| walker |  | 5562 | 24 | python method at src/chronos/chronos2/pipeline.py:47 |  |  | 0.523 |
| walker |  | 5583 | 21 | python method body at src/chronos/chronos2/pipeline.py:43 body 44 |  |  | 0.525 |
| ns | 5639 |  | 86 | Chronos2Model — class declaration + supports flags | 4.3 |  | 0.530 |
| walker |  | 5652 | 69 | python method at src/chronos/chronos2/pipeline.py:1105 |  |  | 0.530 |
| walker |  | 5679 | 27 | python method body at src/chronos/chronos2/pipeline.py:84 body 86 |  |  | 0.536 |
| walker |  | 5769 | 90 | python method at src/chronos/chronos2/pipeline.py:1027 |  |  | 0.536 |
| walker |  | 5836 | 67 | python method doc at src/chronos/chronos2/pipeline.py:1185 |  |  | 0.536 |
| ns | 5903 |  | 264 | Chronos2Model.forward — signature + ModelOutput shape | 4.4 |  | 0.528 |
| walker |  | 5921 | 85 | python decl body at src/chronos/chronos2/trainer.py:17 body 18 |  |  | 0.528 |
| walker |  | 5967 | 46 | python method body at src/chronos/chronos2/trainer.py:33 body 34 |  |  | 0.528 |
| walker |  | 6067 | 100 | python decl at src/chronos/df_utils.py:59 |  |  | 0.528 |
| ns | 6117 |  | 214 | Chronos2Model.forward — group_ids docstring excerpt | 4.5 |  | 0.522 |
| walker |  | 6243 | 176 | python method doc at src/chronos/base.py:252 |  |  | 0.524 |
| walker |  | 6388 | 145 | python method at src/chronos/chronos2/model.py:550 |  |  | 0.524 |
| ns | 6410 |  | 293 | MHA — class + attn-impl branching | 4.6 | 4.1 | 0.511 |
| walker |  | 6533 | 145 | python method at src/chronos/chronos2/pipeline.py:455 |  |  | 0.534 |
| walker |  | 6622 | 89 | python imports in src/chronos/chronos2/config.py |  |  | 0.534 |
| ns | 6673 |  | 263 | RoPE — class + apply_rotary_pos_emb signature | 4.7 | 4.1 | 0.526 |
| walker |  | 6945 | 323 | python decl doc at src/chronos/utils.py:22 |  |  | 0.526 |
| ns | 7040 |  | 367 | Chronos2 dataset — public functions | 4.8 |  | 0.512 |
| ns | 7146 |  | 106 | PreparedInput TypedDict fields | 4.9 | 4.8 | 0.507 |
| ns | 7299 |  | 153 | Chronos2Dataset — class + __init__ signature | 4.10 |  | 0.501 |
| walker |  | 7307 | 362 | python method sigs in src/chronos/chronos2/layers.py |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:25 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:87 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:95 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:111 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:118 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:127 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:135 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:151 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:295 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:318 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:346 |  |  | 0.504 |
| walker |  | 7307 | 0 | python method at src/chronos/chronos2/layers.py:393 |  |  | 0.504 |
| walker |  | 7315 | 8 | python method at src/chronos/chronos2/layers.py:51 |  |  | 0.504 |
| walker |  | 7331 | 16 | python method doc at src/chronos/chronos2/layers.py:51 |  |  | 0.504 |
| walker |  | 7379 | 48 | python method at src/chronos/chronos2/layers.py:34 |  |  | 0.504 |
| walker |  | 7433 | 54 | python method at src/chronos/chronos2/layers.py:353 |  |  | 0.504 |
| walker |  | 7467 | 34 | python method doc at src/chronos/chronos2/layers.py:87 |  |  | 0.504 |
| ns | 7475 |  | 176 | Chronos2Trainer — class + dataloader override | 4.11 |  | 0.512 |
| walker |  | 7526 | 59 | python method at src/chronos/chronos2/layers.py:58 |  |  | 0.517 |
| walker |  | 7587 | 61 | python method at src/chronos/chronos2/layers.py:301 |  |  | 0.517 |
| walker |  | 7648 | 61 | python method at src/chronos/chronos2/layers.py:324 |  |  | 0.517 |
| walker |  | 7727 | 79 | python method at src/chronos/chronos2/layers.py:227 |  |  | 0.517 |
| ns | 7756 |  | 281 | Chronos-Bolt model — class declaration + key components | 4.12 |  | 0.507 |
| walker |  | 7810 | 83 | python method at src/chronos/chronos2/layers.py:372 |  |  | 0.507 |
| walker |  | 7826 | 16 | python method body at src/chronos/base.py:48 body 55 |  |  | 0.507 |
| walker |  | 7978 | 152 | python imports in src/chronos/boto_utils.py |  |  | 0.507 |
| ns | 8160 |  | 404 | Chronos2Pipeline — long-horizon heuristic structure | 4.13 |  | 0.491 |
| walker |  | 8307 | 329 | python decl doc at src/chronos/utils.py:135 |  |  | 0.491 |
| walker |  | 8458 | 151 | python method at src/chronos/chronos2/model.py:618 |  |  | 0.513 |
| ns | 8502 |  | 342 | Chronos-Bolt forward — encoder, decoder, loss outline | 4.14 |  | 0.499 |
| ns | 8580 |  | 78 | test/ directory listing | 5.1 |  | 0.507 |
| walker |  | 8609 | 151 | python method at src/chronos/chronos2/pipeline.py:763 |  |  | 0.507 |
| ns | 8628 |  | 48 | scripts/ + ci/ directory listings | 5.2 |  | 0.504 |
| walker |  | 8671 | 62 | python method at src/chronos/chronos2/layers.py:197 |  |  | 0.504 |
| walker |  | 8734 | 63 | python method at src/chronos/chronos2/layers.py:169 |  |  | 0.504 |
| walker |  | 8797 | 63 | python method at src/chronos/chronos2/pipeline.py:720 |  |  | 0.507 |
| walker |  | 8874 | 77 | python decl names surface in src/chronos/chronos.py |  |  | 0.509 |
| walker |  | 8874 | 0 | python decl at src/chronos/chronos.py:59 |  |  | 0.509 |
| walker |  | 8874 | 0 | python decl at src/chronos/chronos.py:154 |  |  | 0.509 |
| walker |  | 8874 | 0 | python decl at src/chronos/chronos.py:243 |  |  | 0.509 |
| walker |  | 8874 | 0 | python decl at src/chronos/chronos.py:355 |  |  | 0.509 |
| walker |  | 8881 | 7 | python decl at src/chronos/chronos.py:27 |  |  | 0.509 |
| walker |  | 8927 | 46 | python decl doc at src/chronos/chronos.py:27 |  |  | 0.512 |
| ns | 8985 |  | 357 | df_utils — public function names | 5.3 |  | 0.505 |
| walker |  | 8998 | 71 | python decl doc at src/chronos/chronos.py:59 |  |  | 0.505 |
| walker |  | 9099 | 101 | python decl doc at src/chronos/chronos.py:243 |  |  | 0.505 |
| walker |  | 9220 | 121 | python decl doc at src/chronos/chronos.py:355 |  |  | 0.505 |
| walker |  | 9258 | 38 | python class body at src/chronos/chronos.py:355 |  |  | 0.509 |
| ns | 9292 |  | 307 | utils + boto_utils — public function names | 5.4 |  | 0.506 |
| walker |  | 9313 | 55 | python method body at src/chronos/chronos2/layers.py:135 body 136 |  |  | 0.506 |
| ns | 9444 |  | 152 | Test files — all test function names (test_chronos.py) | 5.5 |  | 0.501 |
| ns | 9503 |  | 59 | scripts/training/configs listing | 5.6 |  | 0.498 |
| walker |  | 9786 | 473 | README.md section #10 |  |  | 0.498 |
| ns | 9961 |  | 458 | test_chronos_bolt.py + test_chronos2.py + test_df_utils.py + test_utils.py — test names | 5.7 |  | 0.487 |

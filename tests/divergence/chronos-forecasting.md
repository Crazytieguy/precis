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
| walker |  | 2066 | 10 | python decl at src/chronos/chronos2/config.py:102 |  |  | 0.696 |
| walker |  | 2091 | 25 | python decl at src/chronos/chronos2/config.py:12 |  |  | 0.621 |
| ns | 2091 |  | 386 | README — predict_df example call | 2.6 |  | 0.621 |
| walker |  | 2112 | 21 | python method sigs in src/chronos/chronos2/config.py |  |  | 0.621 |
| walker |  | 2124 | 12 | python method at src/chronos/chronos2/config.py:114 |  |  | 0.621 |
| ns | 2148 |  | 57 | BaseChronosPipeline.predict_fev — signature | 2.7 |  | 0.624 |
| walker |  | 2152 | 28 | python decl names surface in src/chronos/chronos2/pipeline.py |  |  | 0.624 |
| walker |  | 2152 | 0 | python decl at src/chronos/chronos2/pipeline.py:39 |  |  | 0.624 |
| walker |  | 2183 | 31 | python class body at src/chronos/chronos2/pipeline.py:39 |  |  | 0.625 |
| ns | 2273 |  | 125 | PipelineRegistry metaclass | 2.8 |  | 0.611 |
| walker |  | 2301 | 118 | python method at src/chronos/base.py:100 |  |  | 0.647 |
| walker |  | 2311 | 10 | python method body at src/chronos/base.py:100 body 133 |  |  | 0.647 |
| walker |  | 2344 | 33 | README.md section #9 |  |  | 0.647 |
| walker |  | 2347 | 3 | listing of 'ci' |  |  | 0.648 |
| walker |  | 2355 | 8 | README headline in scripts/README.md |  |  | 0.648 |
| walker |  | 2371 | 16 | python method body at src/chronos/chronos2/config.py:114 body 119 |  |  | 0.648 |
| walker |  | 2416 | 45 | python decl names surface in src/chronos/chronos2/trainer.py |  |  | 0.648 |
| walker |  | 2416 | 0 | python decl at src/chronos/chronos2/trainer.py:17 |  |  | 0.648 |
| walker |  | 2416 | 0 | python decl at src/chronos/chronos2/trainer.py:30 |  |  | 0.648 |
| walker |  | 2416 | 0 | python decl at src/chronos/chronos2/trainer.py:40 |  |  | 0.648 |
| walker |  | 2432 | 16 | python decl doc at src/chronos/chronos2/trainer.py:30 |  |  | 0.648 |
| walker |  | 2501 | 69 | python method sigs in src/chronos/chronos2/trainer.py |  |  | 0.648 |
| walker |  | 2501 | 0 | python method at src/chronos/chronos2/trainer.py:33 |  |  | 0.648 |
| walker |  | 2501 | 0 | python method at src/chronos/chronos2/trainer.py:46 |  |  | 0.648 |
| walker |  | 2501 | 0 | python method at src/chronos/chronos2/trainer.py:77 |  |  | 0.648 |
| walker |  | 2557 | 56 | python decl doc at src/chronos/chronos2/trainer.py:40 |  |  | 0.649 |
| ns | 2574 |  | 301 | ChronosPipeline + ChronosBoltPipeline + Chronos2Pipeline — class declarations | 3.1 |  | 0.602 |
| walker |  | 2585 | 28 | python method doc at src/chronos/chronos2/config.py:114 |  |  | 0.603 |
| walker |  | 2766 | 181 | python method at src/chronos/base.py:135 |  |  | 0.660 |
| ns | 2797 |  | 223 | ChronosConfig dataclass — all fields | 3.2 |  | 0.620 |
| walker |  | 2833 | 67 | README.md section #3 |  |  | 0.620 |
| ns | 2882 |  | 85 | ChronosBoltConfig dataclass — all fields | 3.3 |  | 0.606 |
| walker |  | 2915 | 82 | python class body at src/chronos/chronos2/config.py:12 |  |  | 0.607 |
| walker |  | 2990 | 75 | README.md section #1 |  |  | 0.607 |
| walker |  | 3049 | 59 | python method doc at src/chronos/base.py:48 |  |  | 0.607 |
| ns | 3085 |  | 203 | Chronos2ForecastingConfig dataclass — all fields | 3.4 |  | 0.591 |
| walker |  | 3113 | 64 | python method doc at src/chronos/base.py:336 |  |  | 0.591 |
| walker |  | 3225 | 112 | python class body at src/chronos/chronos2/config.py:102 |  |  | 0.634 |
| walker |  | 3285 | 60 | python imports in src/chronos/utils.py |  |  | 0.634 |
| walker |  | 3361 | 76 | python decl names surface in src/chronos/chronos2/model.py |  |  | 0.634 |
| walker |  | 3361 | 0 | python decl at src/chronos/chronos2/model.py:38 |  |  | 0.634 |
| walker |  | 3361 | 0 | python decl at src/chronos/chronos2/model.py:89 |  |  | 0.634 |
| walker |  | 3361 | 0 | python decl at src/chronos/chronos2/model.py:198 |  |  | 0.634 |
| walker |  | 3371 | 10 | python decl at src/chronos/chronos2/model.py:190 |  |  | 0.634 |
| walker |  | 3382 | 11 | python decl at src/chronos/chronos2/model.py:82 |  |  | 0.634 |
| walker |  | 3394 | 12 | python decl at src/chronos/chronos2/model.py:31 |  |  | 0.634 |
| ns | 3413 |  | 328 | Chronos2CoreConfig — class signature + defaults | 3.5 |  | 0.604 |
| walker |  | 3445 | 51 | python class body at src/chronos/chronos2/model.py:31 |  |  | 0.604 |
| ns | 3466 |  | 53 | ChronosTokenizer + MeanScaleUniformBins — class lines | 3.6 |  | 0.600 |
| walker |  | 3509 | 64 | python class body at src/chronos/chronos2/model.py:82 |  |  | 0.600 |
| ns | 3550 |  | 84 | ChronosTokenizer — all abstract method signatures | 3.7 |  | 0.592 |
| walker |  | 3573 | 64 | python class body at src/chronos/chronos2/model.py:198 |  |  | 0.593 |
| walker |  | 3651 | 78 | python class body at src/chronos/chronos2/model.py:190 |  |  | 0.593 |
| walker |  | 3658 | 7 | listing of '.github' |  |  | 0.593 |
| walker |  | 3675 | 17 | listing of '.github/workflows' |  |  | 0.593 |
| walker |  | 3771 | 96 | README.md section #4 |  |  | 0.593 |
| ns | 3807 |  | 257 | ChronosPipeline.predict + predict_quantiles signatures | 3.8 |  | 0.572 |
| ns | 3916 |  | 109 | ChronosBoltPipeline — predict + quantiles property | 3.9 |  | 0.562 |
| walker |  | 3957 | 186 | python method sigs in src/chronos/chronos2/model.py |  |  | 0.562 |
| walker |  | 3957 | 0 | python method at src/chronos/chronos2/model.py:39 |  |  | 0.562 |
| walker |  | 3957 | 0 | python method at src/chronos/chronos2/model.py:90 |  |  | 0.562 |
| walker |  | 3957 | 0 | python method at src/chronos/chronos2/model.py:204 |  |  | 0.562 |
| walker |  | 3957 | 0 | python method at src/chronos/chronos2/model.py:266 |  |  | 0.562 |
| walker |  | 4047 | 90 | python method at src/chronos/chronos2/model.py:134 |  |  | 0.562 |
| ns | 4139 |  | 223 | Chronos2Pipeline — properties (context, patch, prediction, quantiles) | 3.10 |  | 0.542 |
| walker |  | 4162 | 115 | README.md section #2 |  |  | 0.542 |
| walker |  | 4255 | 93 | python method at src/chronos/chronos2/model.py:48 |  |  | 0.542 |
| walker |  | 4289 | 34 | python imports in test/__init__.py |  |  | 0.542 |
| ns | 4296 |  | 157 | Chronos2Pipeline.predict — signature + cross_learning | 3.11 |  | 0.530 |
| walker |  | 4331 | 42 | python decl names surface in src/chronos/df_utils.py |  |  | 0.530 |
| walker |  | 4371 | 40 | python method at src/chronos/chronos2/model.py:98 |  |  | 0.530 |
| walker |  | 4414 | 43 | python method at src/chronos/chronos2/model.py:373 |  |  | 0.530 |
| walker |  | 4462 | 48 | python decl names surface in src/chronos/utils.py |  |  | 0.530 |
| walker |  | 4462 | 0 | python decl at src/chronos/utils.py:11 |  |  | 0.530 |
| ns | 4543 |  | 247 | Chronos2Pipeline.predict_df — signature (covariates-aware) | 3.12 |  | 0.515 |
| walker |  | 4562 | 100 | python imports in src/chronos/df_utils.py |  |  | 0.515 |
| walker |  | 4687 | 125 | python decl names surface in src/chronos/chronos2/layers.py |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:18 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:86 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:110 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:126 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:148 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:294 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:317 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:343 |  |  | 0.516 |
| walker |  | 4687 | 0 | python decl at src/chronos/chronos2/layers.py:369 |  |  | 0.516 |
| walker |  | 4695 | 8 | python decl at src/chronos/chronos2/layers.py:142 |  |  | 0.516 |
| walker |  | 4704 | 9 | python decl doc at src/chronos/chronos2/layers.py:148 |  |  | 0.516 |
| walker |  | 4723 | 19 | python decl doc at src/chronos/chronos2/layers.py:343 |  |  | 0.517 |
| walker |  | 4742 | 19 | python decl doc at src/chronos/chronos2/layers.py:369 |  |  | 0.517 |
| walker |  | 4773 | 31 | python class body at src/chronos/chronos2/layers.py:142 |  |  | 0.517 |
| walker |  | 4858 | 85 | python decl doc at src/chronos/chronos2/layers.py:18 |  |  | 0.517 |
| walker |  | 4870 | 12 | python method body at src/chronos/base.py:48 body 56 |  |  | 0.517 |
| ns | 4908 |  | 365 | Chronos2Pipeline.fit — signature (LoRA + validation) | 3.13 |  | 0.497 |
| walker |  | 4984 | 114 | python decl body at src/chronos/utils.py:11 body 12 |  |  | 0.497 |
| ns | 5154 |  | 246 | Chronos2Pipeline.from_pretrained + save_pretrained | 3.14 |  | 0.486 |
| walker |  | 5265 | 281 | python method sigs in src/chronos/chronos2/pipeline.py |  |  | 0.489 |
| walker |  | 5265 | 0 | python method at src/chronos/chronos2/pipeline.py:43 |  |  | 0.489 |
| walker |  | 5265 | 0 | python method at src/chronos/chronos2/pipeline.py:1224 |  |  | 0.489 |
| walker |  | 5277 | 12 | python method at src/chronos/chronos2/pipeline.py:76 |  |  | 0.490 |
| walker |  | 5289 | 12 | python method at src/chronos/chronos2/pipeline.py:84 |  |  | 0.492 |
| walker |  | 5302 | 13 | python method at src/chronos/chronos2/pipeline.py:80 |  |  | 0.494 |
| ns | 5311 |  | 157 | Chronos2 layers — names of every nn.Module | 4.1 |  | 0.511 |
| walker |  | 5315 | 13 | python method at src/chronos/chronos2/pipeline.py:88 |  |  | 0.513 |
| walker |  | 5328 | 13 | python method at src/chronos/chronos2/pipeline.py:92 |  |  | 0.515 |
| walker |  | 5350 | 22 | python method at src/chronos/chronos2/pipeline.py:1185 |  |  | 0.516 |
| walker |  | 5365 | 15 | python method body at src/chronos/chronos2/pipeline.py:76 body 78 |  |  | 0.519 |
| walker |  | 5380 | 15 | python method body at src/chronos/chronos2/pipeline.py:88 body 90 |  |  | 0.522 |
| walker |  | 5396 | 16 | python method body at src/chronos/chronos2/pipeline.py:80 body 82 |  |  | 0.526 |
| walker |  | 5413 | 17 | python method body at src/chronos/chronos2/pipeline.py:92 body 94 |  |  | 0.530 |
| walker |  | 5446 | 33 | python method doc at src/chronos/chronos2/pipeline.py:1224 |  |  | 0.533 |
| walker |  | 5466 | 20 | python method body at src/chronos/chronos2/pipeline.py:1224 body 1228 |  |  | 0.535 |
| walker |  | 5490 | 24 | python method at src/chronos/chronos2/pipeline.py:47 |  |  | 0.535 |
| walker |  | 5511 | 21 | python method body at src/chronos/chronos2/pipeline.py:43 body 44 |  |  | 0.537 |
| ns | 5553 |  | 242 | Chronos2 encoder block + encoder — class lines | 4.2 |  | 0.525 |
| walker |  | 5580 | 69 | python method at src/chronos/chronos2/pipeline.py:1105 |  |  | 0.525 |
| walker |  | 5607 | 27 | python method body at src/chronos/chronos2/pipeline.py:84 body 86 |  |  | 0.531 |
| ns | 5639 |  | 86 | Chronos2Model — class declaration + supports flags | 4.3 |  | 0.536 |
| walker |  | 5697 | 90 | python method at src/chronos/chronos2/pipeline.py:1027 |  |  | 0.536 |
| walker |  | 5764 | 67 | python method doc at src/chronos/chronos2/pipeline.py:1185 |  |  | 0.536 |
| walker |  | 5856 | 92 | python decl at src/chronos/utils.py:135 |  |  | 0.536 |
| ns | 5903 |  | 264 | Chronos2Model.forward — signature + ModelOutput shape | 4.4 |  | 0.527 |
| walker |  | 5941 | 85 | python decl body at src/chronos/chronos2/trainer.py:17 body 18 |  |  | 0.527 |
| ns | 6117 |  | 214 | Chronos2Model.forward — group_ids docstring excerpt | 4.5 |  | 0.521 |
| walker |  | 6223 | 282 | python decl doc at src/chronos/utils.py:135 |  |  | 0.521 |
| walker |  | 6269 | 46 | python method body at src/chronos/chronos2/trainer.py:33 body 34 |  |  | 0.521 |
| walker |  | 6368 | 99 | python decl at src/chronos/utils.py:22 |  |  | 0.521 |
| ns | 6410 |  | 293 | MHA — class + attn-impl branching | 4.6 | 4.1 | 0.508 |
| walker |  | 6644 | 276 | python decl doc at src/chronos/utils.py:22 |  |  | 0.508 |
| ns | 6673 |  | 263 | RoPE — class + apply_rotary_pos_emb signature | 4.7 | 4.1 | 0.501 |
| walker |  | 6820 | 176 | python method doc at src/chronos/base.py:252 |  |  | 0.503 |
| walker |  | 6965 | 145 | python method at src/chronos/chronos2/model.py:550 |  |  | 0.503 |
| ns | 7040 |  | 367 | Chronos2 dataset — public functions | 4.8 |  | 0.490 |
| walker |  | 7110 | 145 | python method at src/chronos/chronos2/pipeline.py:455 |  |  | 0.512 |
| ns | 7146 |  | 106 | PreparedInput TypedDict fields | 4.9 | 4.8 | 0.507 |
| walker |  | 7199 | 89 | python imports in src/chronos/chronos2/config.py |  |  | 0.507 |
| ns | 7299 |  | 153 | Chronos2Dataset — class + __init__ signature | 4.10 |  | 0.501 |
| ns | 7475 |  | 176 | Chronos2Trainer — class + dataloader override | 4.11 |  | 0.508 |
| walker |  | 7561 | 362 | python method sigs in src/chronos/chronos2/layers.py |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:25 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:87 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:95 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:111 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:118 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:127 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:135 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:151 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:295 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:318 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:346 |  |  | 0.511 |
| walker |  | 7561 | 0 | python method at src/chronos/chronos2/layers.py:393 |  |  | 0.511 |
| walker |  | 7569 | 8 | python method at src/chronos/chronos2/layers.py:51 |  |  | 0.511 |
| walker |  | 7585 | 16 | python method doc at src/chronos/chronos2/layers.py:51 |  |  | 0.511 |
| walker |  | 7633 | 48 | python method at src/chronos/chronos2/layers.py:34 |  |  | 0.511 |
| walker |  | 7687 | 54 | python method at src/chronos/chronos2/layers.py:353 |  |  | 0.511 |
| walker |  | 7721 | 34 | python method doc at src/chronos/chronos2/layers.py:87 |  |  | 0.511 |
| ns | 7756 |  | 281 | Chronos-Bolt model — class declaration + key components | 4.12 |  | 0.501 |
| walker |  | 7780 | 59 | python method at src/chronos/chronos2/layers.py:58 |  |  | 0.507 |
| walker |  | 7841 | 61 | python method at src/chronos/chronos2/layers.py:301 |  |  | 0.507 |
| walker |  | 7902 | 61 | python method at src/chronos/chronos2/layers.py:324 |  |  | 0.507 |
| walker |  | 7981 | 79 | python method at src/chronos/chronos2/layers.py:227 |  |  | 0.507 |
| walker |  | 8064 | 83 | python method at src/chronos/chronos2/layers.py:372 |  |  | 0.507 |
| walker |  | 8080 | 16 | python method body at src/chronos/base.py:48 body 55 |  |  | 0.507 |
| ns | 8160 |  | 404 | Chronos2Pipeline — long-horizon heuristic structure | 4.13 |  | 0.491 |
| walker |  | 8232 | 152 | python imports in src/chronos/boto_utils.py |  |  | 0.491 |
| walker |  | 8383 | 151 | python method at src/chronos/chronos2/model.py:618 |  |  | 0.512 |
| ns | 8502 |  | 342 | Chronos-Bolt forward — encoder, decoder, loss outline | 4.14 |  | 0.499 |
| walker |  | 8534 | 151 | python method at src/chronos/chronos2/pipeline.py:763 |  |  | 0.499 |
| ns | 8580 |  | 78 | test/ directory listing | 5.1 |  | 0.507 |
| walker |  | 8596 | 62 | python method at src/chronos/chronos2/layers.py:197 |  |  | 0.507 |
| ns | 8628 |  | 48 | scripts/ + ci/ directory listings | 5.2 |  | 0.504 |
| walker |  | 8708 | 112 | python decl at src/chronos/df_utils.py:59 |  |  | 0.504 |
| walker |  | 8771 | 63 | python method at src/chronos/chronos2/layers.py:169 |  |  | 0.504 |
| walker |  | 8834 | 63 | python method at src/chronos/chronos2/pipeline.py:720 |  |  | 0.507 |
| walker |  | 8911 | 77 | python decl names surface in src/chronos/chronos.py |  |  | 0.509 |
| walker |  | 8911 | 0 | python decl at src/chronos/chronos.py:59 |  |  | 0.509 |
| walker |  | 8911 | 0 | python decl at src/chronos/chronos.py:154 |  |  | 0.509 |
| walker |  | 8911 | 0 | python decl at src/chronos/chronos.py:243 |  |  | 0.509 |
| walker |  | 8918 | 7 | python decl at src/chronos/chronos.py:27 |  |  | 0.509 |
| walker |  | 8964 | 46 | python decl doc at src/chronos/chronos.py:27 |  |  | 0.512 |
| ns | 8985 |  | 357 | df_utils — public function names | 5.3 |  | 0.505 |
| walker |  | 8993 | 29 | python decl at src/chronos/chronos.py:355 |  |  | 0.505 |
| walker |  | 9064 | 71 | python decl doc at src/chronos/chronos.py:59 |  |  | 0.505 |
| walker |  | 9156 | 92 | python decl doc at src/chronos/chronos.py:355 |  |  | 0.505 |
| walker |  | 9257 | 101 | python decl doc at src/chronos/chronos.py:243 |  |  | 0.505 |
| ns | 9292 |  | 307 | utils + boto_utils — public function names | 5.4 |  | 0.503 |
| walker |  | 9295 | 38 | python class body at src/chronos/chronos.py:355 |  |  | 0.506 |
| walker |  | 9350 | 55 | python method body at src/chronos/chronos2/layers.py:135 body 136 |  |  | 0.506 |
| ns | 9444 |  | 152 | Test files — all test function names (test_chronos.py) | 5.5 |  | 0.501 |
| ns | 9503 |  | 59 | scripts/training/configs listing | 5.6 |  | 0.498 |
| walker |  | 9823 | 473 | README.md section #10 |  |  | 0.498 |
| ns | 9961 |  | 458 | test_chronos_bolt.py + test_chronos2.py + test_df_utils.py + test_utils.py — test names | 5.7 |  | 0.487 |

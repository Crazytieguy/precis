Score(3000)=0.388 I=0.576 C=0.261 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.498/0.553/0.391/0.388/0.445/0.544/0.590

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 54 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 54 |  | 54 | Repository identity: README title + one-sentence purpose | 1.1 |  | 0.000 |
| walker |  | 75 | 21 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.371 |
| walker |  | 78 | 3 | Fs::DirListing { dir: ci } |  |  | 0.371 |
| ns | 104 |  | 50 | Complete root directory listing | 1.2 |  | 0.761 |
| walker |  | 105 | 27 | Fs::DirListing { dir: notebooks } |  |  | 0.762 |
| walker |  | 152 | 47 | Fs::DirListing { dir: src/chronos } |  |  | 0.800 |
| ns | 170 |  | 66 | The three model families, one sentence each | 1.3 |  | 0.722 |
| walker |  | 182 | 30 | Fs::DirListing { dir: src/chronos/chronos2 } |  |  | 0.774 |
| walker |  | 250 | 68 | Toml::Identity { file: pyproject.toml } |  |  | 0.775 |
| ns | 251 |  | 81 | Complete listing of the package source tree | 1.4 |  | 0.766 |
| walker |  | 257 | 7 | Fs::DirListing { dir: .github } |  |  | 0.766 |
| walker |  | 274 | 17 | Fs::DirListing { dir: .github/workflows } |  |  | 0.767 |
| walker |  | 289 | 15 | Code::CodeKey { rung: ModuleDoc, file: src/chronos/__about__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 305 | 16 | Fs::DirListing { dir: scripts } |  |  | 0.767 |
| walker |  | 404 | 99 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.767 |
| ns | 407 |  | 156 | `chronos` package public exports with their defining modules | 1.5 |  | 0.630 |
| walker |  | 424 | 20 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 468 |  | 61 | Minimal forecasting example: import and load a pipeline | 1.6 |  | 0.609 |
| walker |  | 532 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 562 | 30 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.611 |
| ns | 611 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.557 |
| walker |  | 640 | 78 | Fs::DirListing { dir: test } |  |  | 0.557 |
| ns | 809 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.498 |
| ns | 1039 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.436 |
| walker |  | 1066 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.438 |
| walker |  | 1362 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.461 |
| walker |  | 1545 | 183 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.461 |
| walker |  | 1587 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 1609 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.470 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.439 |
| walker |  | 1648 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.439 |
| walker |  | 1666 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.439 |
| walker |  | 1710 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 1812 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.440 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.407 |
| walker |  | 1947 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.409 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.391 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.381 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.361 |
| walker |  | 2415 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.361 |
| walker |  | 2465 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.361 |
| walker |  | 2512 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.362 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.345 |
| walker |  | 2566 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.346 |
| walker |  | 2599 | 33 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.346 |
| walker |  | 2697 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 1, sub: 0, line: 12 } |  |  | 0.347 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.333 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.325 |
| walker |  | 2898 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.378 |
| walker |  | 2906 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.383 |
| walker |  | 2914 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.388 |
| walker |  | 2955 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.388 |
| walker |  | 3020 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.403 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.385 |
| walker |  | 3140 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.407 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.418 |
| walker |  | 3323 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.454 |
| walker |  | 3329 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.454 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.457 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.442 |
| walker |  | 3641 | 312 | Toml::Config { file: pyproject.toml } |  |  | 0.443 |
| walker |  | 3681 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.434 |
| walker |  | 3817 | 136 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 3, sub: 0, line: 102 } |  |  | 0.458 |
| walker |  | 3823 | 6 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.461 |
| walker |  | 3839 | 16 | Code::CodeKey { rung: Body, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.467 |
| walker |  | 3846 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.467 |
| walker |  | 3872 | 26 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.469 |
| walker |  | 3919 | 47 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/trainer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| ns | 3938 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.458 |
| walker |  | 3941 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/trainer.py, decl: 2, sub: 0, line: 30 } |  |  | 0.458 |
| walker |  | 3988 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/trainer.py, decl: 4, sub: 0, line: 40 } |  |  | 0.459 |
| walker |  | 4004 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/trainer.py, decl: 2, sub: 0, line: 30 } |  |  | 0.459 |
| walker |  | 4086 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| walker |  | 4146 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.459 |
| walker |  | 4154 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.459 |
| ns | 4183 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.444 |
| walker |  | 4189 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.444 |
| walker |  | 4197 | 8 | Code::CodeKey { rung: Body, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.444 |
| walker |  | 4261 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.445 |
| walker |  | 4287 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.445 |
| walker |  | 4397 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.445 |
| ns | 4510 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.425 |
| walker |  | 4541 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.426 |
| walker |  | 4575 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.426 |
| walker |  | 4615 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.426 |
| ns | 4663 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.419 |
| walker |  | 4813 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.436 |
| walker |  | 4821 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.438 |
| walker |  | 4829 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.440 |
| walker |  | 4837 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.443 |
| walker |  | 4847 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.435 |
| ns | 4847 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.435 |
| walker |  | 4965 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.435 |
| walker |  | 5086 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.435 |
| walker |  | 5119 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.435 |
| ns | 5123 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.422 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.418 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.424 |
| walker |  | 5318 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.453 |
| walker |  | 5362 | 44 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.455 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.445 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.461 |
| walker |  | 5748 | 386 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.498 |
| walker |  | 5756 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 5, sub: 0, line: 76 } |  |  | 0.500 |
| walker |  | 5764 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 6, sub: 0, line: 80 } |  |  | 0.502 |
| walker |  | 5772 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 7, sub: 0, line: 84 } |  |  | 0.504 |
| walker |  | 5780 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 8, sub: 0, line: 88 } |  |  | 0.507 |
| walker |  | 5788 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 9, sub: 0, line: 92 } |  |  | 0.509 |
| walker |  | 5797 | 9 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.512 |
| walker |  | 5805 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 4, sub: 0, line: 47 } |  |  | 0.515 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.522 |
| walker |  | 5878 | 73 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.525 |
| walker |  | 5970 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 20, sub: 0, line: 1027 } |  |  | 0.525 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.534 |
| walker |  | 6119 | 149 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 13, sub: 0, line: 455 } |  |  | 0.552 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.544 |
| walker |  | 6272 | 153 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 17, sub: 0, line: 763 } |  |  | 0.544 |
| walker |  | 6318 | 46 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.547 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.538 |
| walker |  | 6353 | 35 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/pipeline.py, decl: 23, sub: 0, line: 1224 } |  |  | 0.538 |
| walker |  | 6418 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.538 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.534 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.527 |
| walker |  | 6656 | 238 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 18, sub: 0, line: 821 } |  |  | 0.544 |
| walker |  | 6731 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 11, sub: 0, line: 372 } |  |  | 0.544 |
| walker |  | 6810 | 79 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 19, sub: 0, line: 957 } |  |  | 0.544 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.532 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.520 |
| walker |  | 7167 | 357 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 10, sub: 0, line: 96 } |  |  | 0.543 |
| walker |  | 7272 | 105 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 14, sub: 0, line: 656 } |  |  | 0.543 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.536 |
| walker |  | 7379 | 107 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 12, sub: 0, line: 407 } |  |  | 0.536 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.532 |
| walker |  | 7446 | 67 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.532 |
| walker |  | 7530 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.533 |
| walker |  | 7539 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.533 |
| walker |  | 7549 | 10 | Markdown::ReadmeHeadline { file: scripts/README.md } |  |  | 0.533 |
| walker |  | 7559 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.533 |
| walker |  | 7569 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.533 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.527 |
| walker |  | 7630 | 61 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 5, sub: 0, line: 48 } |  |  | 0.527 |
| walker |  | 7692 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.527 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.534 |
| walker |  | 7909 | 217 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 2, sub: 0, line: 54 } |  |  | 0.553 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.558 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.558 |
| walker |  | 8024 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 8049 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.561 |
| walker |  | 8093 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.563 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.563 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.565 |
| walker |  | 8164 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.569 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.567 |
| walker |  | 8235 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.577 |
| walker |  | 8311 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.580 |
| walker |  | 8359 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.580 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.578 |
| walker |  | 8444 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.578 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.584 |
| walker |  | 8609 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.594 |
| walker |  | 8617 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.596 |
| walker |  | 8625 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.597 |
| walker |  | 8633 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.599 |
| walker |  | 8641 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.601 |
| walker |  | 8690 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.603 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.597 |
| walker |  | 8751 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.597 |
| walker |  | 8872 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.597 |
| walker |  | 8903 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.597 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.590 |
| walker |  | 9091 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.599 |
| walker |  | 9139 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.599 |
| walker |  | 9187 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.599 |
| walker |  | 9262 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.599 |
| walker |  | 9295 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.599 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.591 |
| walker |  | 9357 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.591 |
| walker |  | 9421 | 64 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.598 |
| walker |  | 9433 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.601 |
| walker |  | 9509 | 76 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.601 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.596 |
| walker |  | 9555 | 46 | Code::CodeKey { rung: Body, file: src/chronos/chronos2/trainer.py, decl: 3, sub: 0, line: 33 } |  |  | 0.596 |
| walker |  | 9709 | 154 | Code::CodeKey { rung: Names, file: src/chronos/boto_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 9754 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 7, sub: 0, line: 98 } |  |  | 0.603 |
| walker |  | 9813 | 59 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 5, sub: 0, line: 25 } |  |  | 0.605 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.597 |
| walker |  | 9878 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 6, sub: 0, line: 53 } |  |  | 0.599 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.598 |
| walker |  | 9971 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.600 |
| walker |  | 10000 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.600 |

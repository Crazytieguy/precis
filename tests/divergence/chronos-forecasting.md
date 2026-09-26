Score(3000)=0.461 I=0.590 C=0.361 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.551/0.431/0.461/0.464/0.481/0.625

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | Repository identity: README title + one-sentence purpose | 1.1 |  | 0.000 |
| walker |  | 71 | 21 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.371 |
| walker |  | 98 | 27 | Fs::DirListing { dir: notebooks } |  |  | 0.371 |
| ns | 104 |  | 50 | Complete root directory listing | 1.2 |  | 0.761 |
| walker |  | 166 | 68 | Toml::Identity { file: pyproject.toml } |  |  | 0.762 |
| ns | 170 |  | 66 | The three model families, one sentence each | 1.3 |  | 0.687 |
| walker |  | 216 | 50 | Fs::DirListing { dir: src/chronos } |  |  | 0.722 |
| walker |  | 231 | 15 | Code::CodeKey { rung: ModuleDoc, file: src/chronos/__about__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| ns | 250 |  | 80 | Complete listing of the package source tree | 1.4 |  | 0.578 |
| walker |  | 261 | 30 | Fs::DirListing { dir: src/chronos/chronos2 } |  |  | 0.766 |
| walker |  | 268 | 7 | Fs::DirListing { dir: .github } |  |  | 0.766 |
| walker |  | 285 | 17 | Fs::DirListing { dir: .github/workflows } |  |  | 0.767 |
| walker |  | 301 | 16 | Fs::DirListing { dir: scripts } |  |  | 0.767 |
| walker |  | 308 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.767 |
| walker |  | 317 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.768 |
| ns | 406 |  | 156 | `chronos` package public exports with their defining modules | 1.5 |  | 0.631 |
| walker |  | 416 | 99 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.631 |
| walker |  | 432 | 16 | Fs::DirListing { dir: scripts/evaluation } |  |  | 0.631 |
| ns | 467 |  | 61 | Minimal forecasting example: import and load a pipeline | 1.6 |  | 0.610 |
| ns | 610 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.555 |
| walker |  | 728 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| ns | 808 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.626 |
| ns | 1038 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.548 |
| walker |  | 1196 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 1274 | 78 | Fs::DirListing { dir: test } |  |  | 0.549 |
| walker |  | 1382 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 1447 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.459 |
| ns | 1629 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.429 |
| walker |  | 1808 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.431 |
| ns | 1847 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.399 |
| walker |  | 1850 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.402 |
| walker |  | 1872 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.407 |
| walker |  | 1911 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.407 |
| ns | 2030 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.389 |
| walker |  | 2112 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.450 |
| walker |  | 2120 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.456 |
| walker |  | 2128 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.462 |
| walker |  | 2169 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.462 |
| ns | 2172 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.453 |
| walker |  | 2234 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.461 |
| ns | 2353 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.436 |
| walker |  | 2354 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.462 |
| walker |  | 2537 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.507 |
| ns | 2540 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.484 |
| walker |  | 2555 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.484 |
| walker |  | 2563 | 8 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.484 |
| walker |  | 2573 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.484 |
| walker |  | 2583 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.485 |
| ns | 2719 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.465 |
| ns | 2840 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.461 |
| walker |  | 2975 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.461 |
| ns | 3114 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.441 |
| ns | 3222 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.450 |
| walker |  | 3244 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.450 |
| walker |  | 3294 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 3341 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.450 |
| ns | 3354 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.440 |
| walker |  | 3395 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.441 |
| walker |  | 3510 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 3535 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.442 |
| walker |  | 3579 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.443 |
| ns | 3580 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.428 |
| walker |  | 3650 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.438 |
| ns | 3719 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.429 |
| walker |  | 3721 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.452 |
| walker |  | 3797 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.452 |
| walker |  | 3845 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.452 |
| walker |  | 3930 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.452 |
| ns | 3937 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.441 |
| walker |  | 4095 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.462 |
| walker |  | 4103 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.465 |
| walker |  | 4111 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.468 |
| walker |  | 4119 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.471 |
| walker |  | 4127 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.475 |
| walker |  | 4176 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.479 |
| ns | 4182 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.464 |
| walker |  | 4237 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.464 |
| walker |  | 4358 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.464 |
| ns | 4509 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.443 |
| walker |  | 4546 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.444 |
| walker |  | 4594 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.444 |
| walker |  | 4642 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.444 |
| ns | 4662 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.437 |
| walker |  | 4717 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.437 |
| walker |  | 4799 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.437 |
| ns | 4846 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.426 |
| walker |  | 4859 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.427 |
| walker |  | 4867 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.427 |
| walker |  | 4902 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.427 |
| walker |  | 4966 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.427 |
| walker |  | 4992 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.427 |
| walker |  | 5102 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.427 |
| ns | 5122 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.415 |
| ns | 5231 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.410 |
| walker |  | 5301 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.440 |
| ns | 5307 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.437 |
| walker |  | 5445 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.438 |
| walker |  | 5479 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.438 |
| ns | 5487 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.454 |
| walker |  | 5519 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.454 |
| ns | 5700 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.469 |
| walker |  | 5717 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.482 |
| walker |  | 5725 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.484 |
| walker |  | 5733 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.486 |
| walker |  | 5741 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.488 |
| walker |  | 5751 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.490 |
| ns | 5851 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.498 |
| walker |  | 5869 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.498 |
| walker |  | 5990 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.498 |
| walker |  | 6021 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.498 |
| walker |  | 6065 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| ns | 6069 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.488 |
| walker |  | 6149 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.488 |
| ns | 6239 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.481 |
| walker |  | 6251 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.486 |
| ns | 6345 |  | 106 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.484 |
| walker |  | 6386 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.501 |
| walker |  | 6419 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.501 |
| walker |  | 6452 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.501 |
| ns | 6458 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.497 |
| walker |  | 6464 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.497 |
| ns | 6621 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.490 |
| walker |  | 6819 | 355 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.573 |
| walker |  | 6827 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.573 |
| walker |  | 6835 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.573 |
| walker |  | 6875 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 6917 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.561 |
| ns | 7152 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.548 |
| walker |  | 7261 | 386 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.580 |
| walker |  | 7269 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 4, sub: 0, line: 47 } |  |  | 0.582 |
| walker |  | 7277 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 5, sub: 0, line: 76 } |  |  | 0.584 |
| walker |  | 7285 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 6, sub: 0, line: 80 } |  |  | 0.586 |
| walker |  | 7293 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 7, sub: 0, line: 84 } |  |  | 0.588 |
| walker |  | 7301 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 8, sub: 0, line: 88 } |  |  | 0.591 |
| ns | 7306 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.583 |
| walker |  | 7309 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 9, sub: 0, line: 92 } |  |  | 0.586 |
| walker |  | 7318 | 9 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.588 |
| walker |  | 7364 | 46 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.590 |
| walker |  | 7429 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.590 |
| ns | 7437 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.586 |
| walker |  | 7502 | 73 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.588 |
| walker |  | 7577 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 11, sub: 0, line: 372 } |  |  | 0.588 |
| ns | 7612 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.581 |
| walker |  | 7656 | 79 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 19, sub: 0, line: 957 } |  |  | 0.581 |
| walker |  | 7748 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 20, sub: 0, line: 1027 } |  |  | 0.581 |
| ns | 7806 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.586 |
| walker |  | 7853 | 105 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 14, sub: 0, line: 656 } |  |  | 0.586 |
| ns | 7943 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.593 |
| walker |  | 7960 | 107 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 12, sub: 0, line: 407 } |  |  | 0.593 |
| ns | 7970 |  | 27 | Notebook inventory | 6.3 |  | 0.594 |
| ns | 8095 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.598 |
| walker |  | 8109 | 149 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 13, sub: 0, line: 455 } |  |  | 0.613 |
| ns | 8128 |  | 33 | Complete `.github` listing | 6.5 |  | 0.616 |
| ns | 8215 |  | 87 | CI gate commands | 6.6 |  | 0.613 |
| walker |  | 8262 | 153 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 17, sub: 0, line: 763 } |  |  | 0.613 |
| ns | 8369 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.611 |
| walker |  | 8500 | 238 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 18, sub: 0, line: 821 } |  |  | 0.626 |
| ns | 8541 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.617 |
| ns | 8728 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.611 |
| walker |  | 8857 | 357 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 10, sub: 0, line: 96 } |  |  | 0.631 |
| walker |  | 8892 | 35 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/pipeline.py, decl: 23, sub: 0, line: 1224 } |  |  | 0.631 |
| walker |  | 8985 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| ns | 8989 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.625 |
| walker |  | 9014 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.625 |
| walker |  | 9071 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.626 |
| walker |  | 9134 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.629 |
| walker |  | 9169 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.630 |
| walker |  | 9210 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.630 |
| walker |  | 9280 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.632 |
| ns | 9314 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.623 |
| walker |  | 9364 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.624 |
| walker |  | 9449 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.624 |
| ns | 9537 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.619 |
| walker |  | 9541 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.619 |
| walker |  | 9718 | 177 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.639 |
| walker |  | 9763 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.639 |
| walker |  | 9854 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.639 |
| ns | 9864 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.631 |
| walker |  | 9951 | 97 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.631 |
| ns | 9952 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.630 |
| ns | 9978 |  | 26 | Licensing statement | 6.15 |  | 0.629 |

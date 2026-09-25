Score(3000)=0.386 I=0.556 C=0.268 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.628/0.551/0.383/0.386/0.491/0.493/0.542

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
| walker |  | 167 | 15 | Code::CodeKey { rung: ModuleDoc, file: src/chronos/__about__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| ns | 170 |  | 66 | The three model families, one sentence each | 1.3 |  | 0.722 |
| walker |  | 197 | 30 | Fs::DirListing { dir: src/chronos/chronos2 } |  |  | 0.774 |
| ns | 251 |  | 81 | Complete listing of the package source tree | 1.4 |  | 0.765 |
| walker |  | 265 | 68 | Toml::Identity { file: pyproject.toml } |  |  | 0.766 |
| walker |  | 272 | 7 | Fs::DirListing { dir: .github } |  |  | 0.766 |
| walker |  | 289 | 17 | Fs::DirListing { dir: .github/workflows } |  |  | 0.767 |
| walker |  | 305 | 16 | Fs::DirListing { dir: scripts } |  |  | 0.767 |
| walker |  | 312 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.767 |
| walker |  | 321 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.768 |
| ns | 407 |  | 156 | `chronos` package public exports with their defining modules | 1.5 |  | 0.631 |
| walker |  | 420 | 99 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.631 |
| walker |  | 436 | 16 | Fs::DirListing { dir: scripts/evaluation } |  |  | 0.632 |
| ns | 468 |  | 61 | Minimal forecasting example: import and load a pipeline | 1.6 |  | 0.610 |
| ns | 611 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.556 |
| walker |  | 732 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| ns | 809 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.627 |
| walker |  | 810 | 78 | Fs::DirListing { dir: test } |  |  | 0.628 |
| ns | 1039 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.549 |
| walker |  | 1278 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.549 |
| walker |  | 1386 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 1392 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.551 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.460 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.429 |
| walker |  | 1818 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.431 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.400 |
| walker |  | 1900 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 1960 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.400 |
| walker |  | 1968 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.400 |
| walker |  | 2003 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.400 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.383 |
| walker |  | 2067 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.383 |
| walker |  | 2093 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.383 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.374 |
| walker |  | 2203 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.374 |
| walker |  | 2347 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.375 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.355 |
| walker |  | 2381 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.355 |
| walker |  | 2421 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.355 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.339 |
| walker |  | 2619 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.370 |
| walker |  | 2627 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.374 |
| walker |  | 2635 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.378 |
| walker |  | 2643 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.382 |
| walker |  | 2653 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.387 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.372 |
| walker |  | 2771 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.372 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.362 |
| walker |  | 2892 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.362 |
| walker |  | 3091 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.413 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.395 |
| walker |  | 3133 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.397 |
| walker |  | 3155 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.400 |
| walker |  | 3194 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.400 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.410 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.401 |
| walker |  | 3395 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.443 |
| walker |  | 3403 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.447 |
| walker |  | 3411 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.451 |
| walker |  | 3452 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.451 |
| walker |  | 3517 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.464 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.448 |
| walker |  | 3637 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.466 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.457 |
| walker |  | 3820 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.488 |
| walker |  | 3838 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.488 |
| ns | 3938 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.475 |
| walker |  | 3953 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| walker |  | 3978 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.477 |
| walker |  | 4022 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.477 |
| walker |  | 4093 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.485 |
| walker |  | 4164 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.506 |
| ns | 4183 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.490 |
| walker |  | 4240 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.491 |
| walker |  | 4288 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.491 |
| walker |  | 4373 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.491 |
| ns | 4510 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.468 |
| walker |  | 4538 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.487 |
| walker |  | 4546 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.489 |
| walker |  | 4554 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.492 |
| walker |  | 4562 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.495 |
| walker |  | 4570 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.498 |
| walker |  | 4619 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.502 |
| ns | 4663 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.494 |
| walker |  | 4680 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.494 |
| walker |  | 4801 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.494 |
| ns | 4847 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.481 |
| walker |  | 4989 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.483 |
| walker |  | 5037 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.483 |
| walker |  | 5085 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.483 |
| ns | 5123 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.469 |
| walker |  | 5160 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.469 |
| walker |  | 5170 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.469 |
| walker |  | 5180 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.469 |
| walker |  | 5192 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.470 |
| walker |  | 5223 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.470 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.465 |
| walker |  | 5267 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.461 |
| walker |  | 5351 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.461 |
| walker |  | 5453 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.461 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.476 |
| walker |  | 5588 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.477 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.491 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.485 |
| walker |  | 5980 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.485 |
| walker |  | 6013 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.485 |
| walker |  | 6046 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.485 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.497 |
| walker |  | 6139 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 6168 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.499 |
| walker |  | 6225 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.500 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.493 |
| walker |  | 6288 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.497 |
| walker |  | 6323 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.498 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.495 |
| walker |  | 6364 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.496 |
| walker |  | 6434 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.498 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.494 |
| walker |  | 6518 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.496 |
| walker |  | 6603 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.496 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.488 |
| walker |  | 6695 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.488 |
| walker |  | 6872 | 177 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.513 |
| walker |  | 6917 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.513 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.502 |
| walker |  | 7008 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.502 |
| walker |  | 7105 | 97 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.502 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.491 |
| walker |  | 7224 | 119 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 15, sub: 0, line: 315 } |  |  | 0.491 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.485 |
| walker |  | 7371 | 147 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 19, sub: 0, line: 550 } |  |  | 0.485 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.481 |
| walker |  | 7524 | 153 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 20, sub: 0, line: 618 } |  |  | 0.493 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.487 |
| walker |  | 7793 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.487 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.491 |
| walker |  | 7843 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7890 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.496 |
| walker |  | 7944 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.503 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.509 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.509 |
| walker |  | 7977 | 33 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 8075 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.519 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.522 |
| walker |  | 8211 | 136 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 3, sub: 0, line: 102 } |  |  | 0.533 |
| walker |  | 8217 | 6 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.531 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.531 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.529 |
| walker |  | 8434 | 217 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 2, sub: 0, line: 54 } |  |  | 0.546 |
| walker |  | 8449 | 15 | Fs::DirListing { dir: test/dummy-chronos-model } |  |  | 0.551 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.543 |
| walker |  | 8672 | 223 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/dataset.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 8701 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 11, sub: 0, line: 466 } |  |  | 0.554 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.548 |
| walker |  | 8731 | 30 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 7, sub: 0, line: 302 } |  |  | 0.548 |
| walker |  | 8771 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 4, sub: 0, line: 50 } |  |  | 0.548 |
| walker |  | 8822 | 51 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 10, sub: 0, line: 406 } |  |  | 0.548 |
| walker |  | 8889 | 67 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 5, sub: 0, line: 219 } |  |  | 0.548 |
| walker |  | 8963 | 74 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 9, sub: 0, line: 370 } |  |  | 0.548 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.542 |
| walker |  | 9045 | 82 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 2, sub: 0, line: 23 } |  |  | 0.545 |
| walker |  | 9173 | 128 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 12, sub: 0, line: 472 } |  |  | 0.553 |
| walker |  | 9304 | 131 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 13, sub: 0, line: 511 } |  |  | 0.553 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.546 |
| walker |  | 9318 | 14 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 15, sub: 0, line: 609 } |  |  | 0.546 |
| walker |  | 9334 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 6, sub: 0, line: 263 } |  |  | 0.546 |
| walker |  | 9351 | 17 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 2, sub: 0, line: 23 } |  |  | 0.548 |
| walker |  | 9367 | 16 | Code::CodeKey { rung: Body, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.551 |
| walker |  | 9407 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.546 |
| walker |  | 9793 | 386 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.571 |
| walker |  | 9801 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 4, sub: 0, line: 47 } |  |  | 0.573 |
| walker |  | 9809 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 5, sub: 0, line: 76 } |  |  | 0.574 |
| walker |  | 9817 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 6, sub: 0, line: 80 } |  |  | 0.576 |
| walker |  | 9825 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 7, sub: 0, line: 84 } |  |  | 0.578 |
| walker |  | 9833 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 8, sub: 0, line: 88 } |  |  | 0.579 |
| walker |  | 9841 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 9, sub: 0, line: 92 } |  |  | 0.581 |
| walker |  | 9850 | 9 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.583 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.576 |
| walker |  | 9896 | 46 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.578 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.577 |
| walker |  | 9961 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.577 |
| walker |  | 9972 | 11 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.579 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.578 |

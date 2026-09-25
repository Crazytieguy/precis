Score(3000)=0.319 I=0.543 C=0.187 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.498/0.439/0.382/0.319/0.408/0.481/0.527

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
| walker |  | 454 | 30 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 468 |  | 61 | Minimal forecasting example: import and load a pipeline | 1.6 |  | 0.609 |
| walker |  | 532 | 78 | Fs::DirListing { dir: test } |  |  | 0.610 |
| ns | 611 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.555 |
| walker |  | 640 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| ns | 809 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.498 |
| ns | 1039 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.436 |
| walker |  | 1066 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.438 |
| walker |  | 1249 | 183 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.439 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.366 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.342 |
| walker |  | 1717 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.342 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.317 |
| walker |  | 2013 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.399 |
| walker |  | 2019 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.399 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.382 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.372 |
| walker |  | 2331 | 312 | Toml::Config { file: pyproject.toml } |  |  | 0.374 |
| walker |  | 2338 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.374 |
| walker |  | 2347 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.374 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.354 |
| walker |  | 2357 | 10 | Markdown::ReadmeHeadline { file: scripts/README.md } |  |  | 0.354 |
| walker |  | 2367 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.354 |
| walker |  | 2377 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.355 |
| walker |  | 2389 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.355 |
| walker |  | 2471 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.356 |
| walker |  | 2531 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.356 |
| walker |  | 2539 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.356 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.340 |
| walker |  | 2574 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.340 |
| walker |  | 2582 | 8 | Code::CodeKey { rung: Body, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.340 |
| walker |  | 2646 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.340 |
| walker |  | 2672 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.340 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.326 |
| walker |  | 2782 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.326 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.318 |
| walker |  | 2926 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.319 |
| walker |  | 2960 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.319 |
| walker |  | 3000 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.319 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.305 |
| walker |  | 3198 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.333 |
| walker |  | 3206 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.336 |
| walker |  | 3214 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.340 |
| walker |  | 3222 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.343 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.356 |
| walker |  | 3232 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.361 |
| walker |  | 3350 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.361 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.352 |
| walker |  | 3471 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.352 |
| walker |  | 3504 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.352 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.341 |
| walker |  | 3703 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.385 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.377 |
| walker |  | 3747 | 44 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.380 |
| walker |  | 3809 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.380 |
| walker |  | 3885 | 76 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.380 |
| walker |  | 3927 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.382 |
| ns | 3938 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.372 |
| walker |  | 3949 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.375 |
| walker |  | 3988 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.375 |
| walker |  | 4006 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.375 |
| ns | 4183 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.363 |
| walker |  | 4207 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.401 |
| walker |  | 4215 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.404 |
| walker |  | 4223 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.408 |
| walker |  | 4264 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.408 |
| walker |  | 4329 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.419 |
| walker |  | 4449 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.436 |
| ns | 4510 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.416 |
| walker |  | 4632 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.444 |
| ns | 4663 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.436 |
| walker |  | 4693 | 61 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 5, sub: 0, line: 48 } |  |  | 0.436 |
| walker |  | 4757 | 64 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.452 |
| ns | 4847 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.441 |
| walker |  | 4872 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 4897 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.442 |
| walker |  | 4941 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.442 |
| walker |  | 5012 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.449 |
| walker |  | 5083 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.468 |
| ns | 5123 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.454 |
| walker |  | 5159 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.455 |
| walker |  | 5207 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.455 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.450 |
| walker |  | 5292 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.450 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.446 |
| walker |  | 5457 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.463 |
| walker |  | 5465 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.465 |
| walker |  | 5473 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.468 |
| walker |  | 5481 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.470 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.470 |
| walker |  | 5489 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.473 |
| walker |  | 5538 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.476 |
| walker |  | 5599 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.476 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.491 |
| walker |  | 5720 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.491 |
| walker |  | 5751 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.491 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.484 |
| walker |  | 5939 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.498 |
| walker |  | 5987 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.498 |
| walker |  | 6035 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.498 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.488 |
| walker |  | 6110 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.488 |
| walker |  | 6143 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.488 |
| walker |  | 6205 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.488 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.481 |
| walker |  | 6249 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.475 |
| walker |  | 6351 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.479 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.476 |
| walker |  | 6486 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.492 |
| walker |  | 6570 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.493 |
| walker |  | 6585 | 15 | Fs::DirListing { dir: test/dummy-chronos-model } |  |  | 0.494 |
| walker |  | 6601 | 16 | Fs::DirListing { dir: scripts/evaluation } |  |  | 0.499 |
| walker |  | 6609 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.499 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.491 |
| walker |  | 6659 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 6706 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.497 |
| walker |  | 6760 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.505 |
| walker |  | 6866 | 106 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.505 |
| walker |  | 6876 | 10 | Fs::DirListing { dir: scripts/evaluation/configs } |  |  | 0.508 |
| walker |  | 6884 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.508 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.497 |
| walker |  | 6998 | 114 | Code::CodeKey { rung: Body, file: src/chronos/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.497 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.486 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.480 |
| walker |  | 7390 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.480 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.476 |
| walker |  | 7521 | 131 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.486 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.480 |
| walker |  | 7675 | 154 | Code::CodeKey { rung: Names, file: src/chronos/boto_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 7720 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 7, sub: 0, line: 98 } |  |  | 0.489 |
| walker |  | 7779 | 59 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 5, sub: 0, line: 25 } |  |  | 0.491 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.499 |
| walker |  | 7844 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 6, sub: 0, line: 53 } |  |  | 0.502 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.508 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.509 |
| walker |  | 7973 | 129 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/layers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 8002 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 21, sub: 0, line: 294 } |  |  | 0.514 |
| walker |  | 8031 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 24, sub: 0, line: 317 } |  |  | 0.514 |
| walker |  | 8062 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.514 |
| walker |  | 8093 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.514 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.523 |
| walker |  | 8130 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 15, sub: 0, line: 142 } |  |  | 0.526 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.526 |
| walker |  | 8167 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 29, sub: 0, line: 353 } |  |  | 0.526 |
| walker |  | 8207 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 6, sub: 0, line: 86 } |  |  | 0.526 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.524 |
| walker |  | 8247 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 9, sub: 0, line: 110 } |  |  | 0.524 |
| walker |  | 8287 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 12, sub: 0, line: 126 } |  |  | 0.524 |
| walker |  | 8350 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 23, sub: 0, line: 301 } |  |  | 0.524 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.522 |
| walker |  | 8413 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 26, sub: 0, line: 324 } |  |  | 0.522 |
| walker |  | 8478 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.525 |
| walker |  | 8487 | 9 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.525 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.532 |
| walker |  | 8568 | 81 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 20, sub: 0, line: 227 } |  |  | 0.532 |
| walker |  | 8650 | 82 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.535 |
| walker |  | 8658 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.537 |
| walker |  | 8668 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 3, sub: 0, line: 34 } |  |  | 0.537 |
| walker |  | 8725 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 5, sub: 0, line: 58 } |  |  | 0.538 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.533 |
| walker |  | 8810 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 31, sub: 0, line: 372 } |  |  | 0.533 |
| walker |  | 8826 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.533 |
| walker |  | 8845 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.533 |
| walker |  | 8864 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.533 |
| walker |  | 8900 | 36 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 7, sub: 0, line: 87 } |  |  | 0.533 |
| walker |  | 8964 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 19, sub: 0, line: 197 } |  |  | 0.533 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.527 |
| walker |  | 9029 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 18, sub: 0, line: 169 } |  |  | 0.527 |
| walker |  | 9298 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.520 |
| walker |  | 9388 | 90 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.520 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.516 |
| walker |  | 9546 | 158 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.516 |
| walker |  | 9639 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 9668 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.517 |
| walker |  | 9725 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.518 |
| walker |  | 9788 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.520 |
| walker |  | 9858 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.521 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.515 |
| walker |  | 9942 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.516 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.515 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.516 |

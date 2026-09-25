Score(3000)=0.362 I=0.551 C=0.239 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.498/0.438/0.383/0.362/0.470/0.482/0.520

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
| walker |  | 1108 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.436 |
| walker |  | 1114 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.436 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.364 |
| walker |  | 1540 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.366 |
| walker |  | 1547 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.366 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.341 |
| walker |  | 1730 | 183 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.342 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.317 |
| walker |  | 2026 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.399 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.382 |
| walker |  | 2035 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.382 |
| walker |  | 2045 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.382 |
| walker |  | 2055 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.383 |
| walker |  | 2137 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.373 |
| walker |  | 2197 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.374 |
| walker |  | 2205 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.374 |
| walker |  | 2240 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.374 |
| walker |  | 2248 | 8 | Code::CodeKey { rung: Body, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.374 |
| walker |  | 2312 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.374 |
| walker |  | 2338 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.374 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.355 |
| walker |  | 2448 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.355 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.338 |
| walker |  | 2592 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.339 |
| walker |  | 2626 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.339 |
| walker |  | 2666 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.339 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.326 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.317 |
| walker |  | 2864 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.346 |
| walker |  | 2872 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.350 |
| walker |  | 2880 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.354 |
| walker |  | 2888 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.358 |
| walker |  | 2898 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.362 |
| walker |  | 3016 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.362 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.346 |
| walker |  | 3137 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.346 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.359 |
| walker |  | 3336 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.406 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.396 |
| walker |  | 3369 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.396 |
| walker |  | 3413 | 44 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.399 |
| walker |  | 3475 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.399 |
| walker |  | 3517 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.401 |
| walker |  | 3539 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.404 |
| walker |  | 3578 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.404 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.391 |
| walker |  | 3596 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.391 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.383 |
| walker |  | 3797 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.423 |
| walker |  | 3805 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.427 |
| walker |  | 3813 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.431 |
| walker |  | 3854 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.431 |
| walker |  | 3919 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.442 |
| ns | 3938 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.431 |
| walker |  | 4039 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.448 |
| ns | 4183 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.434 |
| walker |  | 4222 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.463 |
| walker |  | 4283 | 61 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 5, sub: 0, line: 48 } |  |  | 0.463 |
| walker |  | 4347 | 64 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.479 |
| walker |  | 4462 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 4487 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.480 |
| ns | 4510 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.459 |
| walker |  | 4531 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.459 |
| walker |  | 4602 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.467 |
| ns | 4663 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.459 |
| walker |  | 4673 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.478 |
| walker |  | 4749 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.478 |
| walker |  | 4797 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.478 |
| ns | 4847 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.466 |
| walker |  | 4882 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.466 |
| walker |  | 5047 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.484 |
| walker |  | 5055 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.486 |
| walker |  | 5063 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.489 |
| walker |  | 5071 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.491 |
| walker |  | 5079 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.495 |
| ns | 5123 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.480 |
| walker |  | 5128 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.484 |
| walker |  | 5189 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.484 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.478 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.475 |
| walker |  | 5310 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.475 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.475 |
| walker |  | 5498 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.489 |
| walker |  | 5546 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.489 |
| walker |  | 5594 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.489 |
| walker |  | 5669 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.489 |
| walker |  | 5700 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.489 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.503 |
| walker |  | 5733 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.503 |
| walker |  | 5795 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.503 |
| walker |  | 5807 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.503 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.496 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.486 |
| walker |  | 6119 | 312 | Toml::Config { file: pyproject.toml } |  |  | 0.488 |
| walker |  | 6163 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.481 |
| walker |  | 6247 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.482 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.475 |
| walker |  | 6349 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.480 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.476 |
| walker |  | 6484 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.493 |
| walker |  | 6560 | 76 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.493 |
| walker |  | 6610 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.487 |
| walker |  | 6657 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.491 |
| walker |  | 6711 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.499 |
| walker |  | 6726 | 15 | Fs::DirListing { dir: test/dummy-chronos-model } |  |  | 0.500 |
| walker |  | 6742 | 16 | Fs::DirListing { dir: scripts/evaluation } |  |  | 0.505 |
| walker |  | 6750 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.505 |
| walker |  | 6904 | 154 | Code::CodeKey { rung: Names, file: src/chronos/boto_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.499 |
| walker |  | 6949 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 7, sub: 0, line: 98 } |  |  | 0.503 |
| walker |  | 7008 | 59 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 5, sub: 0, line: 25 } |  |  | 0.506 |
| walker |  | 7073 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 6, sub: 0, line: 53 } |  |  | 0.509 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.497 |
| walker |  | 7202 | 129 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/layers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 7231 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 21, sub: 0, line: 294 } |  |  | 0.504 |
| walker |  | 7260 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 24, sub: 0, line: 317 } |  |  | 0.504 |
| walker |  | 7291 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.504 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.498 |
| walker |  | 7322 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.498 |
| walker |  | 7359 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 15, sub: 0, line: 142 } |  |  | 0.499 |
| walker |  | 7396 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 29, sub: 0, line: 353 } |  |  | 0.499 |
| walker |  | 7436 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 6, sub: 0, line: 86 } |  |  | 0.499 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.495 |
| walker |  | 7476 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 9, sub: 0, line: 110 } |  |  | 0.495 |
| walker |  | 7516 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 12, sub: 0, line: 126 } |  |  | 0.495 |
| walker |  | 7579 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 23, sub: 0, line: 301 } |  |  | 0.495 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.489 |
| walker |  | 7642 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 26, sub: 0, line: 324 } |  |  | 0.489 |
| walker |  | 7707 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.492 |
| walker |  | 7716 | 9 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.492 |
| walker |  | 7780 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 19, sub: 0, line: 197 } |  |  | 0.492 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.500 |
| walker |  | 7845 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 18, sub: 0, line: 169 } |  |  | 0.500 |
| walker |  | 7926 | 81 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 20, sub: 0, line: 227 } |  |  | 0.500 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.506 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.507 |
| walker |  | 8008 | 82 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.511 |
| walker |  | 8016 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.512 |
| walker |  | 8026 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 3, sub: 0, line: 34 } |  |  | 0.512 |
| walker |  | 8083 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 5, sub: 0, line: 58 } |  |  | 0.514 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.522 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.525 |
| walker |  | 8168 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 31, sub: 0, line: 372 } |  |  | 0.525 |
| walker |  | 8184 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.525 |
| walker |  | 8203 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.525 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.523 |
| walker |  | 8222 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.523 |
| walker |  | 8258 | 36 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 7, sub: 0, line: 87 } |  |  | 0.523 |
| walker |  | 8364 | 106 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.523 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.521 |
| walker |  | 8374 | 10 | Fs::DirListing { dir: scripts/evaluation/configs } |  |  | 0.523 |
| walker |  | 8382 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.523 |
| walker |  | 8496 | 114 | Code::CodeKey { rung: Body, file: src/chronos/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.523 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.530 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.525 |
| walker |  | 8888 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.525 |
| walker |  | 8981 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.520 |
| walker |  | 9010 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.520 |
| walker |  | 9067 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.521 |
| walker |  | 9130 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.523 |
| walker |  | 9165 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.524 |
| walker |  | 9206 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.525 |
| walker |  | 9276 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.526 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.519 |
| walker |  | 9360 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.520 |
| walker |  | 9445 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.520 |
| walker |  | 9537 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.520 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.516 |
| walker |  | 9714 | 177 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.533 |
| walker |  | 9759 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.533 |
| walker |  | 9850 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.533 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.527 |
| walker |  | 9947 | 97 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.527 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.526 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.527 |
| walker |  | 9990 | 43 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 15, sub: 0, line: 315 } |  |  | 0.527 |

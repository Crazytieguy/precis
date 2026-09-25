Score(3000)=0.386 I=0.555 C=0.268 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.627/0.551/0.383/0.386/0.491/0.493/0.538

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 54 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 54 |  | 54 | Repository identity: README title + one-sentence purpose | 1.1 |  | 0.000 |
| walker |  | 75 | 21 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.371 |
| walker |  | 78 | 3 | Fs::DirListing { dir: ci } |  |  | 0.371 |
| ns | 104 |  | 50 | Complete root directory listing | 1.2 |  | 0.761 |
| walker |  | 105 | 27 | Fs::DirListing { dir: notebooks } |  |  | 0.762 |
| ns | 170 |  | 66 | The three model families, one sentence each | 1.3 |  | 0.687 |
| walker |  | 173 | 68 | Toml::Identity { file: pyproject.toml } |  |  | 0.687 |
| walker |  | 220 | 47 | Fs::DirListing { dir: src/chronos } |  |  | 0.722 |
| walker |  | 235 | 15 | Code::CodeKey { rung: ModuleDoc, file: src/chronos/__about__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| ns | 251 |  | 81 | Complete listing of the package source tree | 1.4 |  | 0.578 |
| walker |  | 265 | 30 | Fs::DirListing { dir: src/chronos/chronos2 } |  |  | 0.766 |
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
| ns | 1039 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.548 |
| walker |  | 1200 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 1278 | 78 | Fs::DirListing { dir: test } |  |  | 0.549 |
| walker |  | 1386 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.459 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.429 |
| walker |  | 1812 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.431 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.399 |
| walker |  | 1894 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 1954 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.400 |
| walker |  | 1962 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.400 |
| walker |  | 1997 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.400 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.383 |
| walker |  | 2061 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.383 |
| walker |  | 2087 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.383 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.373 |
| walker |  | 2197 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.373 |
| walker |  | 2341 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.375 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.355 |
| walker |  | 2375 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.355 |
| walker |  | 2415 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.355 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.339 |
| walker |  | 2613 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.370 |
| walker |  | 2621 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.374 |
| walker |  | 2629 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.377 |
| walker |  | 2637 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.382 |
| walker |  | 2647 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.387 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.372 |
| walker |  | 2765 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.372 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.362 |
| walker |  | 2886 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.362 |
| walker |  | 3085 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.413 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.395 |
| walker |  | 3127 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.397 |
| walker |  | 3149 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.400 |
| walker |  | 3188 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.400 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.410 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.401 |
| walker |  | 3389 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.443 |
| walker |  | 3397 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.447 |
| walker |  | 3405 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.451 |
| walker |  | 3446 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.451 |
| walker |  | 3511 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.464 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.448 |
| walker |  | 3631 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.466 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.457 |
| walker |  | 3814 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.487 |
| walker |  | 3832 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.487 |
| walker |  | 3838 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.488 |
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
| walker |  | 5211 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.469 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.464 |
| walker |  | 5255 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.464 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.461 |
| walker |  | 5339 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.461 |
| walker |  | 5441 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.461 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.475 |
| walker |  | 5576 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.477 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.491 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.485 |
| walker |  | 5968 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.485 |
| walker |  | 6001 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.485 |
| walker |  | 6034 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.485 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.497 |
| walker |  | 6127 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 6156 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.499 |
| walker |  | 6213 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.500 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.493 |
| walker |  | 6276 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.496 |
| walker |  | 6311 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.497 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.495 |
| walker |  | 6352 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.496 |
| walker |  | 6422 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.497 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.494 |
| walker |  | 6506 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.495 |
| walker |  | 6591 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.495 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.488 |
| walker |  | 6683 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.488 |
| walker |  | 6860 | 177 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.513 |
| walker |  | 6905 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.513 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.501 |
| walker |  | 6996 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.501 |
| walker |  | 7093 | 97 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.501 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.490 |
| walker |  | 7212 | 119 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 15, sub: 0, line: 315 } |  |  | 0.490 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.484 |
| walker |  | 7359 | 147 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 19, sub: 0, line: 550 } |  |  | 0.484 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.480 |
| walker |  | 7512 | 153 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 20, sub: 0, line: 618 } |  |  | 0.493 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.487 |
| walker |  | 7781 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.487 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.491 |
| walker |  | 7831 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7878 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.495 |
| walker |  | 7932 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.502 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.508 |
| walker |  | 7965 | 33 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.509 |
| walker |  | 8063 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 1, sub: 0, line: 12 } |  |  | 0.515 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.516 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.519 |
| walker |  | 8199 | 136 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 3, sub: 0, line: 102 } |  |  | 0.530 |
| walker |  | 8205 | 6 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.531 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.529 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.526 |
| walker |  | 8422 | 217 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/config.py, decl: 2, sub: 0, line: 54 } |  |  | 0.544 |
| walker |  | 8434 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.546 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.538 |
| walker |  | 8657 | 223 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/dataset.py, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 8686 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 11, sub: 0, line: 466 } |  |  | 0.549 |
| walker |  | 8716 | 30 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 7, sub: 0, line: 302 } |  |  | 0.549 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.543 |
| walker |  | 8756 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 4, sub: 0, line: 50 } |  |  | 0.543 |
| walker |  | 8807 | 51 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 10, sub: 0, line: 406 } |  |  | 0.543 |
| walker |  | 8874 | 67 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 5, sub: 0, line: 219 } |  |  | 0.543 |
| walker |  | 8948 | 74 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 9, sub: 0, line: 370 } |  |  | 0.543 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.537 |
| walker |  | 9030 | 82 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 2, sub: 0, line: 23 } |  |  | 0.540 |
| walker |  | 9158 | 128 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 12, sub: 0, line: 472 } |  |  | 0.549 |
| walker |  | 9289 | 131 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/dataset.py, decl: 13, sub: 0, line: 511 } |  |  | 0.549 |
| walker |  | 9303 | 14 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 15, sub: 0, line: 609 } |  |  | 0.549 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.541 |
| walker |  | 9319 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 6, sub: 0, line: 263 } |  |  | 0.541 |
| walker |  | 9336 | 17 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/dataset.py, decl: 2, sub: 0, line: 23 } |  |  | 0.543 |
| walker |  | 9352 | 16 | Code::CodeKey { rung: Body, file: src/chronos/chronos2/config.py, decl: 4, sub: 0, line: 114 } |  |  | 0.546 |
| walker |  | 9392 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.542 |
| walker |  | 9778 | 386 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.567 |
| walker |  | 9786 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 4, sub: 0, line: 47 } |  |  | 0.568 |
| walker |  | 9794 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 5, sub: 0, line: 76 } |  |  | 0.570 |
| walker |  | 9802 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 6, sub: 0, line: 80 } |  |  | 0.572 |
| walker |  | 9810 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 7, sub: 0, line: 84 } |  |  | 0.573 |
| walker |  | 9818 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 8, sub: 0, line: 88 } |  |  | 0.575 |
| walker |  | 9826 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 9, sub: 0, line: 92 } |  |  | 0.577 |
| walker |  | 9835 | 9 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.579 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.571 |
| walker |  | 9881 | 46 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.573 |
| walker |  | 9946 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.573 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.572 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.572 |
| walker |  | 9994 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.574 |

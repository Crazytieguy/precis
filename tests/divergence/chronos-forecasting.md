Score(3000)=0.382 I=0.554 C=0.263 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.551/0.383/0.382/0.482/0.502/0.579

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
| ns | 468 |  | 61 | Minimal forecasting example: import and load a pipeline | 1.6 |  | 0.608 |
| ns | 611 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.554 |
| walker |  | 700 | 296 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 778 | 78 | Fs::DirListing { dir: test } |  |  | 0.701 |
| ns | 809 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.626 |
| ns | 1039 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.548 |
| walker |  | 1246 | 468 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 1354 | 108 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 1360 | 6 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.550 |
| ns | 1448 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.458 |
| ns | 1630 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.428 |
| walker |  | 1786 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.430 |
| walker |  | 1793 | 7 | Fs::DirListing { dir: scripts/training } |  |  | 0.430 |
| walker |  | 1802 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.431 |
| walker |  | 1812 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.431 |
| walker |  | 1822 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.431 |
| ns | 1848 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.400 |
| walker |  | 1904 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 1964 | 60 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.400 |
| walker |  | 1972 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.400 |
| walker |  | 2007 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.400 |
| walker |  | 2015 | 8 | Code::CodeKey { rung: Body, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.400 |
| ns | 2031 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.383 |
| walker |  | 2079 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.383 |
| walker |  | 2105 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.383 |
| ns | 2173 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.374 |
| walker |  | 2215 | 110 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.374 |
| ns | 2354 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.354 |
| walker |  | 2359 | 144 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.355 |
| walker |  | 2393 | 34 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.355 |
| walker |  | 2433 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.355 |
| ns | 2541 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.339 |
| walker |  | 2631 | 198 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.370 |
| walker |  | 2639 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.374 |
| walker |  | 2647 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.378 |
| walker |  | 2655 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.382 |
| walker |  | 2665 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.387 |
| ns | 2720 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.372 |
| walker |  | 2783 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.372 |
| ns | 2841 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.362 |
| walker |  | 2904 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.362 |
| walker |  | 3103 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.413 |
| ns | 3115 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.395 |
| walker |  | 3136 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.395 |
| walker |  | 3180 | 44 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.398 |
| ns | 3223 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.408 |
| walker |  | 3242 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.408 |
| walker |  | 3284 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 3306 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.413 |
| walker |  | 3345 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.413 |
| ns | 3355 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.404 |
| walker |  | 3363 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.404 |
| walker |  | 3564 | 201 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.446 |
| walker |  | 3572 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.450 |
| walker |  | 3580 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.454 |
| ns | 3581 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.439 |
| walker |  | 3621 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.439 |
| walker |  | 3686 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.451 |
| ns | 3720 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.442 |
| walker |  | 3806 | 120 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.459 |
| ns | 3938 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.448 |
| walker |  | 3989 | 183 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.478 |
| walker |  | 4050 | 61 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 5, sub: 0, line: 48 } |  |  | 0.478 |
| walker |  | 4114 | 64 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.495 |
| ns | 4183 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.479 |
| walker |  | 4229 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 4254 | 25 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.480 |
| walker |  | 4298 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.480 |
| walker |  | 4369 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.488 |
| walker |  | 4440 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.508 |
| ns | 4510 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.485 |
| walker |  | 4516 | 76 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.486 |
| walker |  | 4564 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.486 |
| walker |  | 4649 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.486 |
| ns | 4663 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.478 |
| walker |  | 4814 | 165 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.496 |
| walker |  | 4822 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.498 |
| walker |  | 4830 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.501 |
| walker |  | 4838 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.503 |
| walker |  | 4846 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.506 |
| ns | 4847 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.494 |
| walker |  | 4895 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.497 |
| walker |  | 4956 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.497 |
| walker |  | 5077 | 121 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.497 |
| ns | 5123 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.483 |
| ns | 5232 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.478 |
| walker |  | 5265 | 188 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.479 |
| ns | 5308 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.476 |
| walker |  | 5313 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.476 |
| walker |  | 5361 | 48 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.476 |
| walker |  | 5436 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.476 |
| walker |  | 5467 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.476 |
| ns | 5488 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.489 |
| walker |  | 5500 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.489 |
| walker |  | 5562 | 62 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.489 |
| walker |  | 5574 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.489 |
| walker |  | 5618 | 44 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| ns | 5701 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.502 |
| walker |  | 5702 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.502 |
| walker |  | 5804 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.503 |
| ns | 5852 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.496 |
| walker |  | 5939 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.498 |
| ns | 6070 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.509 |
| ns | 6240 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.502 |
| walker |  | 6331 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.502 |
| ns | 6347 |  | 107 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.495 |
| walker |  | 6407 | 76 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.495 |
| ns | 6460 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.491 |
| ns | 6623 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.484 |
| walker |  | 6676 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.484 |
| walker |  | 6726 | 50 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 6773 | 47 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.489 |
| walker |  | 6827 | 54 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.497 |
| walker |  | 6842 | 15 | Fs::DirListing { dir: test/dummy-chronos-model } |  |  | 0.498 |
| walker |  | 6858 | 16 | Fs::DirListing { dir: scripts/evaluation } |  |  | 0.503 |
| walker |  | 6866 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.503 |
| ns | 6919 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.492 |
| ns | 7154 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.481 |
| walker |  | 7282 | 416 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 7308 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.554 |
| walker |  | 7436 | 154 | Code::CodeKey { rung: Names, file: src/chronos/boto_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| ns | 7439 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.555 |
| walker |  | 7481 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 7, sub: 0, line: 98 } |  |  | 0.559 |
| walker |  | 7540 | 59 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 5, sub: 0, line: 25 } |  |  | 0.562 |
| walker |  | 7605 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 6, sub: 0, line: 53 } |  |  | 0.566 |
| ns | 7614 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.559 |
| walker |  | 7734 | 129 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/layers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 7763 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 21, sub: 0, line: 294 } |  |  | 0.566 |
| walker |  | 7792 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 24, sub: 0, line: 317 } |  |  | 0.566 |
| ns | 7808 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.570 |
| walker |  | 7823 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.570 |
| walker |  | 7854 | 31 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.570 |
| walker |  | 7891 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 15, sub: 0, line: 142 } |  |  | 0.571 |
| walker |  | 7928 | 37 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 29, sub: 0, line: 353 } |  |  | 0.571 |
| ns | 7945 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.578 |
| walker |  | 7968 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 6, sub: 0, line: 86 } |  |  | 0.578 |
| ns | 7972 |  | 27 | Notebook inventory | 6.3 |  | 0.579 |
| walker |  | 8008 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 9, sub: 0, line: 110 } |  |  | 0.579 |
| walker |  | 8048 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 12, sub: 0, line: 126 } |  |  | 0.579 |
| ns | 8097 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.589 |
| walker |  | 8111 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 23, sub: 0, line: 301 } |  |  | 0.589 |
| ns | 8130 |  | 33 | Complete `.github` listing | 6.5 |  | 0.593 |
| walker |  | 8174 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 26, sub: 0, line: 324 } |  |  | 0.593 |
| ns | 8217 |  | 87 | CI gate commands | 6.6 |  | 0.590 |
| walker |  | 8239 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.593 |
| walker |  | 8248 | 9 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 16, sub: 0, line: 148 } |  |  | 0.593 |
| walker |  | 8312 | 64 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 19, sub: 0, line: 197 } |  |  | 0.593 |
| ns | 8371 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.591 |
| walker |  | 8377 | 65 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 18, sub: 0, line: 169 } |  |  | 0.591 |
| walker |  | 8458 | 81 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 20, sub: 0, line: 227 } |  |  | 0.591 |
| walker |  | 8540 | 82 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.595 |
| ns | 8543 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.586 |
| walker |  | 8548 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.588 |
| walker |  | 8558 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 3, sub: 0, line: 34 } |  |  | 0.588 |
| walker |  | 8615 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 5, sub: 0, line: 58 } |  |  | 0.589 |
| walker |  | 8700 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/layers.py, decl: 31, sub: 0, line: 372 } |  |  | 0.589 |
| walker |  | 8716 | 16 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 4, sub: 0, line: 51 } |  |  | 0.589 |
| ns | 8730 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.584 |
| walker |  | 8735 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 27, sub: 0, line: 343 } |  |  | 0.584 |
| walker |  | 8754 | 19 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 30, sub: 0, line: 369 } |  |  | 0.584 |
| walker |  | 8790 | 36 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/layers.py, decl: 7, sub: 0, line: 87 } |  |  | 0.584 |
| walker |  | 8896 | 106 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.584 |
| walker |  | 8906 | 10 | Fs::DirListing { dir: scripts/evaluation/configs } |  |  | 0.586 |
| walker |  | 8914 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.586 |
| ns | 8991 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.579 |
| walker |  | 9028 | 114 | Code::CodeKey { rung: Body, file: src/chronos/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.579 |
| walker |  | 9121 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 9150 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.581 |
| walker |  | 9207 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.582 |
| walker |  | 9270 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.585 |
| walker |  | 9305 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.586 |
| ns | 9316 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.578 |
| walker |  | 9346 | 41 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.579 |
| walker |  | 9416 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.580 |
| walker |  | 9500 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.581 |
| ns | 9539 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.576 |
| walker |  | 9585 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.576 |
| walker |  | 9677 | 92 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.576 |
| walker |  | 9854 | 177 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.596 |
| ns | 9866 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.589 |
| walker |  | 9899 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.589 |
| ns | 9954 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.588 |
| ns | 9980 |  | 26 | Licensing statement | 6.15 |  | 0.587 |
| walker |  | 9990 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.587 |
| walker |  | 9997 | 7 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.587 |

Score(3000)=0.481 I=0.599 C=0.386 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.551/0.424/0.481/0.466/0.551/0.611

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
| walker |  | 2169 | 258 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.451 |
| ns | 2172 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.445 |
| walker |  | 2177 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.450 |
| walker |  | 2185 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.456 |
| walker |  | 2208 | 23 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.456 |
| walker |  | 2265 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.464 |
| ns | 2353 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.439 |
| walker |  | 2368 | 103 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.463 |
| walker |  | 2537 | 169 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.507 |
| ns | 2540 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.484 |
| walker |  | 2555 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.484 |
| walker |  | 2563 | 8 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.484 |
| walker |  | 2631 | 68 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 2671 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.485 |
| walker |  | 2714 | 43 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.486 |
| ns | 2719 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.466 |
| walker |  | 2829 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| ns | 2840 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.464 |
| walker |  | 2865 | 36 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.464 |
| walker |  | 2909 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.464 |
| walker |  | 2980 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.475 |
| walker |  | 3051 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.502 |
| ns | 3114 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.480 |
| walker |  | 3125 | 74 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.480 |
| ns | 3222 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.488 |
| walker |  | 3223 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.488 |
| walker |  | 3249 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.488 |
| ns | 3354 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.477 |
| walker |  | 3465 | 216 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.500 |
| walker |  | 3473 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 23, sub: 0, line: 411 } |  |  | 0.503 |
| walker |  | 3481 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 24, sub: 0, line: 415 } |  |  | 0.507 |
| walker |  | 3489 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 25, sub: 0, line: 419 } |  |  | 0.510 |
| walker |  | 3497 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 29, sub: 0, line: 609 } |  |  | 0.514 |
| walker |  | 3524 | 27 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.518 |
| walker |  | 3573 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.518 |
| ns | 3580 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.501 |
| walker |  | 3677 | 104 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.501 |
| ns | 3719 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.491 |
| walker |  | 3915 | 238 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.492 |
| walker |  | 3935 | 20 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.492 |
| ns | 3937 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.480 |
| walker |  | 3975 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.480 |
| walker |  | 4036 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.480 |
| walker |  | 4118 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| ns | 4182 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.465 |
| walker |  | 4193 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.465 |
| walker |  | 4208 | 15 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.465 |
| walker |  | 4288 | 80 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.466 |
| walker |  | 4296 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 18, sub: 0, line: 261 } |  |  | 0.466 |
| walker |  | 4323 | 27 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.466 |
| walker |  | 4421 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.466 |
| ns | 4509 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.445 |
| walker |  | 4620 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.477 |
| ns | 4662 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.469 |
| walker |  | 4801 | 181 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.471 |
| walker |  | 4818 | 17 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.471 |
| walker |  | 4838 | 20 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.471 |
| ns | 4846 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.459 |
| walker |  | 5065 | 227 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.475 |
| walker |  | 5073 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 23, sub: 0, line: 380 } |  |  | 0.477 |
| walker |  | 5081 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 24, sub: 0, line: 384 } |  |  | 0.479 |
| walker |  | 5089 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 29, sub: 0, line: 535 } |  |  | 0.481 |
| walker |  | 5099 | 10 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 26, sub: 0, line: 398 } |  |  | 0.484 |
| ns | 5122 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.470 |
| walker |  | 5203 | 104 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.470 |
| ns | 5231 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.465 |
| ns | 5307 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.461 |
| walker |  | 5309 | 106 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.461 |
| walker |  | 5319 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.462 |
| walker |  | 5329 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.462 |
| walker |  | 5360 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.462 |
| ns | 5487 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.476 |
| ns | 5700 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.490 |
| walker |  | 5752 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.490 |
| walker |  | 5785 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.490 |
| walker |  | 5818 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.490 |
| ns | 5851 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.498 |
| ns | 6069 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.487 |
| walker |  | 6087 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.487 |
| walker |  | 6099 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.488 |
| ns | 6239 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.481 |
| ns | 6345 |  | 106 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.479 |
| walker |  | 6454 | 355 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 6458 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.556 |
| walker |  | 6462 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.556 |
| walker |  | 6470 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.556 |
| walker |  | 6510 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| ns | 6621 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.549 |
| walker |  | 6677 | 167 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.555 |
| walker |  | 6685 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 4, sub: 0, line: 47 } |  |  | 0.556 |
| walker |  | 6693 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 5, sub: 0, line: 76 } |  |  | 0.557 |
| walker |  | 6701 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 6, sub: 0, line: 80 } |  |  | 0.558 |
| walker |  | 6709 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 7, sub: 0, line: 84 } |  |  | 0.560 |
| walker |  | 6717 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 8, sub: 0, line: 88 } |  |  | 0.561 |
| walker |  | 6725 | 8 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 9, sub: 0, line: 92 } |  |  | 0.562 |
| ns | 6917 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.550 |
| walker |  | 6937 | 212 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 1, line: 39 } |  |  | 0.566 |
| walker |  | 6966 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.568 |
| walker |  | 7018 | 52 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 11, sub: 0, line: 372 } |  |  | 0.568 |
| walker |  | 7071 | 53 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.568 |
| ns | 7152 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.555 |
| walker |  | 7158 | 87 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 12, sub: 0, line: 407 } |  |  | 0.555 |
| walker |  | 7249 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 14, sub: 0, line: 656 } |  |  | 0.555 |
| ns | 7306 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.548 |
| walker |  | 7384 | 135 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 13, sub: 0, line: 455 } |  |  | 0.565 |
| ns | 7437 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.560 |
| walker |  | 7603 | 219 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 2, line: 39 } |  |  | 0.578 |
| walker |  | 7612 | 9 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 22, sub: 0, line: 1185 } |  |  | 0.574 |
| ns | 7612 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.574 |
| walker |  | 7658 | 46 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.576 |
| walker |  | 7720 | 62 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 19, sub: 0, line: 957 } |  |  | 0.576 |
| walker |  | 7793 | 73 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 20, sub: 0, line: 1027 } |  |  | 0.576 |
| ns | 7806 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.581 |
| walker |  | 7926 | 133 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 17, sub: 0, line: 763 } |  |  | 0.581 |
| ns | 7943 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.588 |
| ns | 7970 |  | 27 | Notebook inventory | 6.3 |  | 0.589 |
| ns | 8095 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.594 |
| ns | 8128 |  | 33 | Complete `.github` listing | 6.5 |  | 0.597 |
| walker |  | 8150 | 224 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 18, sub: 0, line: 821 } |  |  | 0.613 |
| ns | 8215 |  | 87 | CI gate commands | 6.6 |  | 0.610 |
| ns | 8369 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.608 |
| walker |  | 8492 | 342 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 10, sub: 0, line: 96 } |  |  | 0.628 |
| walker |  | 8527 | 35 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/pipeline.py, decl: 23, sub: 0, line: 1224 } |  |  | 0.628 |
| ns | 8541 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.619 |
| walker |  | 8620 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 8665 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.621 |
| walker |  | 8722 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.622 |
| ns | 8728 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.615 |
| walker |  | 8791 | 69 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.615 |
| walker |  | 8861 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.616 |
| walker |  | 8945 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.617 |
| ns | 8989 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.610 |
| walker |  | 9047 | 102 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.613 |
| walker |  | 9070 | 23 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.614 |
| walker |  | 9099 | 29 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.615 |
| walker |  | 9176 | 77 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.615 |
| ns | 9314 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.607 |
| walker |  | 9437 | 261 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.627 |
| walker |  | 9457 | 20 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 16, sub: 0, line: 373 } |  |  | 0.627 |
| walker |  | 9531 | 74 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 17, sub: 0, line: 425 } |  |  | 0.627 |
| ns | 9537 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.622 |
| walker |  | 9616 | 85 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 18, sub: 0, line: 499 } |  |  | 0.622 |
| walker |  | 9727 | 111 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 15, sub: 0, line: 315 } |  |  | 0.622 |
| ns | 9864 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.614 |
| walker |  | 9866 | 139 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 19, sub: 0, line: 550 } |  |  | 0.614 |
| ns | 9952 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.613 |
| ns | 9978 |  | 26 | Licensing statement | 6.15 |  | 0.612 |

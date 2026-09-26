Score(3000)=0.504 I=0.601 C=0.423 ns_rows≤3K=18/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.601/0.633/0.507/0.504/0.498/0.575/0.634

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
| walker |  | 574 | 142 | Code::CodeKey { rung: Names, file: src/chronos/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| ns | 610 |  | 143 | README `predict_df` call with every keyword argument annotated | 1.7 |  | 0.667 |
| walker |  | 652 | 78 | Fs::DirListing { dir: test } |  |  | 0.668 |
| ns | 808 |  | 198 | Complete table of published model IDs | 1.8 |  | 0.597 |
| ns | 1038 |  | 230 | `BaseChronosPipeline` complete member roster + `ForecastType` | 2.1 |  | 0.523 |
| walker |  | 1078 | 426 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.525 |
| walker |  | 1120 | 42 | Code::CodeKey { rung: Names, file: src/chronos/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 1142 | 22 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 1, sub: 0, line: 27 } |  |  | 0.536 |
| walker |  | 1181 | 39 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 2, sub: 0, line: 32 } |  |  | 0.536 |
| ns | 1447 |  | 409 | `Chronos2Pipeline` complete member roster | 2.2 |  | 0.447 |
| walker |  | 1463 | 282 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 4, sub: 0, line: 44 } |  |  | 0.545 |
| walker |  | 1486 | 23 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 12, sub: 0, line: 252 } |  |  | 0.545 |
| walker |  | 1535 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 13, sub: 0, line: 336 } |  |  | 0.546 |
| ns | 1629 |  | 182 | `ChronosBoltPipeline` complete member roster | 2.3 |  | 0.510 |
| walker |  | 1638 | 103 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 10, sub: 0, line: 100 } |  |  | 0.512 |
| walker |  | 1807 | 169 | Code::CodeKey { rung: Decl, file: src/chronos/base.py, decl: 11, sub: 0, line: 135 } |  |  | 0.518 |
| walker |  | 1825 | 18 | Code::CodeKey { rung: Doc, file: src/chronos/base.py, decl: 3, sub: 0, line: 35 } |  |  | 0.518 |
| walker |  | 1833 | 8 | Fs::DirListing { dir: ci/evaluate } |  |  | 0.518 |
| ns | 1847 |  | 218 | `ChronosPipeline` complete member roster + class docstring | 2.4 |  | 0.480 |
| walker |  | 1901 | 68 | Code::CodeKey { rung: Names, file: src/chronos/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 1941 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 3, sub: 0, line: 135 } |  |  | 0.481 |
| walker |  | 1984 | 43 | Code::CodeKey { rung: Decl, file: src/chronos/utils.py, decl: 2, sub: 0, line: 22 } |  |  | 0.482 |
| ns | 2030 |  | 183 | `BaseChronosPipeline.predict_df` full signature | 2.5 | 2.1 | 0.507 |
| walker |  | 2099 | 115 | Code::CodeKey { rung: Names, file: src/chronos/chronos_bolt.py, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 2135 | 36 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 11, sub: 0, line: 113 } |  |  | 0.508 |
| ns | 2172 |  | 142 | `BaseChronosPipeline.predict` / `predict_quantiles` full signatures | 2.6 | 2.1 | 0.521 |
| walker |  | 2179 | 44 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 4, sub: 0, line: 50 } |  |  | 0.521 |
| walker |  | 2250 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 3, sub: 0, line: 42 } |  |  | 0.522 |
| walker |  | 2321 | 71 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 2, sub: 0, line: 32 } |  |  | 0.526 |
| ns | 2353 |  | 181 | `ChronosConfig` complete field list | 2.7 |  | 0.498 |
| walker |  | 2395 | 74 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 12, sub: 0, line: 114 } |  |  | 0.498 |
| walker |  | 2493 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.499 |
| walker |  | 2519 | 26 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 9, sub: 0, line: 81 } |  |  | 0.499 |
| ns | 2540 |  | 187 | `ChronosBoltConfig` and `ChronosBoltOutput` complete field lists | 2.8 |  | 0.514 |
| ns | 2719 |  | 179 | `Chronos2ForecastingConfig` fields + `editable_fields` | 2.9 |  | 0.494 |
| walker |  | 2757 | 238 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 14, sub: 0, line: 147 } |  |  | 0.495 |
| walker |  | 2777 | 20 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 17, sub: 0, line: 240 } |  |  | 0.495 |
| walker |  | 2817 | 40 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 20, sub: 0, line: 364 } |  |  | 0.495 |
| ns | 2840 |  | 121 | `BaseChronosPipeline.from_pretrained` signature + docstring | 2.10 | 2.1 | 0.490 |
| walker |  | 2878 | 61 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 18, sub: 0, line: 294 } |  |  | 0.490 |
| ns | 3114 |  | 274 | `from_pretrained` dispatch body: the `PipelineRegistry` mechanism | 2.11 | 2.10 | 0.468 |
| walker |  | 3136 | 258 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 21, sub: 0, line: 403 } |  |  | 0.512 |
| walker |  | 3153 | 17 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 26, sub: 0, line: 423 } |  |  | 0.512 |
| walker |  | 3202 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 27, sub: 0, line: 463 } |  |  | 0.512 |
| ns | 3222 |  | 108 | `chronos.chronos2` subpackage exports | 3.1 |  | 0.506 |
| walker |  | 3306 | 104 | Code::CodeKey { rung: Decl, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.506 |
| ns | 3354 |  | 132 | `Chronos2CoreConfig` declaration, docstring lede and HF attribute map | 3.2 |  | 0.494 |
| walker |  | 3388 | 82 | Code::CodeKey { rung: Names, file: src/chronos/chronos.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 3463 | 75 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 5, sub: 0, line: 59 } |  |  | 0.495 |
| walker |  | 3478 | 15 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 6, sub: 0, line: 68 } |  |  | 0.495 |
| walker |  | 3566 | 88 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 16, sub: 0, line: 243 } |  |  | 0.495 |
| ns | 3580 |  | 226 | `Chronos2CoreConfig.__init__` full hyperparameter defaults | 3.3 | 3.2 | 0.479 |
| walker |  | 3593 | 27 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 19, sub: 0, line: 265 } |  |  | 0.479 |
| walker |  | 3691 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 20, sub: 0, line: 292 } |  |  | 0.479 |
| ns | 3719 |  | 139 | `Chronos2Pipeline.predict` full signature | 3.4 | 2.2 | 0.469 |
| walker |  | 3890 | 199 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.506 |
| ns | 3937 |  | 218 | `Chronos2Pipeline.predict_df` signature | 3.5 | 2.2 | 0.493 |
| walker |  | 4071 | 181 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 9, sub: 0, line: 154 } |  |  | 0.495 |
| walker |  | 4088 | 17 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 12, sub: 0, line: 198 } |  |  | 0.495 |
| walker |  | 4108 | 20 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 11, sub: 0, line: 170 } |  |  | 0.495 |
| ns | 4182 |  | 245 | `Chronos2Pipeline.fit` signature — in-package fine-tuning | 3.6 | 2.2 | 0.479 |
| walker |  | 4369 | 261 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 21, sub: 0, line: 355 } |  |  | 0.506 |
| walker |  | 4473 | 104 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.506 |
| ns | 4509 |  | 327 | `chronos2/model.py` complete class + method roster | 3.7 |  | 0.483 |
| walker |  | 4579 | 106 | Code::CodeKey { rung: Decl, file: src/chronos/chronos.py, decl: 27, sub: 0, line: 430 } |  |  | 0.483 |
| walker |  | 4589 | 10 | Fs::DirListing { dir: test/dummy-chronos-bolt-model } |  |  | 0.484 |
| walker |  | 4599 | 10 | Fs::DirListing { dir: test/dummy-chronos2-model } |  |  | 0.484 |
| ns | 4662 |  | 153 | `Chronos2Model.forward` full signature | 3.8 | 3.7 | 0.476 |
| ns | 4846 |  | 184 | `chronos2/layers.py` complete class roster | 3.9 |  | 0.464 |
| walker |  | 4991 | 392 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.464 |
| ns | 5122 |  | 276 | `chronos2/dataset.py` complete top-level symbol roster | 3.10 |  | 0.451 |
| ns | 5231 |  | 109 | `PreparedInput` TypedDict — the internal batch element schema | 3.11 | 3.10 | 0.446 |
| walker |  | 5260 | 269 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.446 |
| walker |  | 5291 | 31 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 7, sub: 0, line: 71 } |  |  | 0.446 |
| walker |  | 5303 | 12 | Fs::DirListing { dir: test/dummy-chronos2-lora } |  |  | 0.446 |
| ns | 5307 |  | 76 | `chronos2/trainer.py` complete roster | 3.12 |  | 0.443 |
| ns | 5487 |  | 180 | `chronos_bolt.py` complete component roster | 3.13 |  | 0.458 |
| walker |  | 5658 | 355 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.540 |
| walker |  | 5691 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 28, sub: 0, line: 513 } |  |  | 0.540 |
| ns | 5700 |  | 213 | `chronos.py` complete component roster: tokenizer and model wrapper | 3.14 |  | 0.557 |
| walker |  | 5724 | 33 | Code::CodeKey { rung: Doc, file: src/chronos/chronos_bolt.py, decl: 28, sub: 0, line: 559 } |  |  | 0.557 |
| walker |  | 5739 | 15 | Fs::DirListing { dir: test/dummy-chronos-model } |  |  | 0.557 |
| walker |  | 5747 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 6, sub: 0, line: 58 } |  |  | 0.557 |
| ns | 5851 |  | 151 | `utils.py` complete public roster with signatures | 4.1 |  | 0.566 |
| walker |  | 5878 | 131 | Code::CodeKey { rung: Names, file: src/chronos/df_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5941 | 63 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 1, sub: 0, line: 16 } |  |  | 0.566 |
| walker |  | 6014 | 73 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 2, sub: 0, line: 59 } |  |  | 0.566 |
| ns | 6069 |  | 218 | `df_utils.py` complete function roster with signature heads | 4.2 |  | 0.565 |
| walker |  | 6112 | 98 | Code::CodeKey { rung: Decl, file: src/chronos/df_utils.py, decl: 3, sub: 0, line: 199 } |  |  | 0.581 |
| ns | 6239 |  | 170 | `boto_utils.py` module constants + complete function roster | 4.3 |  | 0.572 |
| walker |  | 6293 | 181 | Code::CodeKey { rung: Names, file: src/chronos/boto_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 6331 | 38 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 7, sub: 0, line: 98 } |  |  | 0.584 |
| ns | 6345 |  | 106 | Complete listing of `scripts/` and `ci/` | 5.1 |  | 0.580 |
| walker |  | 6380 | 49 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 5, sub: 0, line: 25 } |  |  | 0.584 |
| walker |  | 6435 | 55 | Code::CodeKey { rung: Decl, file: src/chronos/boto_utils.py, decl: 6, sub: 0, line: 53 } |  |  | 0.587 |
| walker |  | 6443 | 8 | Code::CodeKey { rung: Body, file: src/chronos/base.py, decl: 7, sub: 0, line: 62 } |  |  | 0.587 |
| ns | 6458 |  | 113 | `scripts/README.md` section headings + staleness warning | 5.2 |  | 0.583 |
| walker |  | 6487 | 44 | Code::CodeKey { rung: Doc, file: src/chronos/chronos.py, decl: 2, sub: 0, line: 27 } |  |  | 0.584 |
| ns | 6621 |  | 163 | `scripts/training/train.py` complete top-level roster | 5.3 |  | 0.576 |
| ns | 6917 |  | 296 | `train.py main()` complete CLI/config parameter list with defaults | 5.4 | 5.3 | 0.563 |
| walker |  | 6953 | 466 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.588 |
| walker |  | 7017 | 64 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 7057 | 40 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/pipeline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| ns | 7152 |  | 235 | `evaluate.py` roster and the three benchmark subcommands | 5.5 |  | 0.579 |
| walker |  | 7247 | 190 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 0, line: 39 } |  |  | 0.589 |
| ns | 7306 |  | 154 | Benchmark config schema: head of `in-domain.yaml` + CI backtest config | 5.6 |  | 0.582 |
| ns | 7437 |  | 131 | `kernel-synth.py` roster + CLI entry point | 5.7 |  | 0.577 |
| walker |  | 7460 | 213 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 1, line: 39 } |  |  | 0.593 |
| walker |  | 7512 | 52 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 11, sub: 0, line: 372 } |  |  | 0.593 |
| walker |  | 7565 | 53 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 15, sub: 0, line: 720 } |  |  | 0.593 |
| ns | 7612 |  | 175 | `agg-relative-score.py` scoring function and command signature | 5.8 |  | 0.586 |
| walker |  | 7652 | 87 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 12, sub: 0, line: 407 } |  |  | 0.586 |
| walker |  | 7743 | 91 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 14, sub: 0, line: 656 } |  |  | 0.586 |
| ns | 7806 |  | 194 | `pyproject.toml`: package identity, Python floor and runtime dependencies | 6.1 |  | 0.590 |
| walker |  | 7868 | 125 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 13, sub: 0, line: 455 } |  |  | 0.604 |
| ns | 7943 |  | 137 | `pyproject.toml`: the four optional-dependency extras | 6.2 |  | 0.610 |
| ns | 7970 |  | 27 | Notebook inventory | 6.3 |  | 0.611 |
| ns | 8095 |  | 125 | Complete listing of `test/` including the dummy checkpoint fixtures | 6.4 |  | 0.620 |
| ns | 8128 |  | 33 | Complete `.github` listing | 6.5 |  | 0.623 |
| walker |  | 8149 | 281 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 2, sub: 2, line: 39 } |  |  | 0.648 |
| walker |  | 8170 | 21 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 16, sub: 0, line: 749 } |  |  | 0.648 |
| walker |  | 8205 | 35 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 21, sub: 0, line: 1105 } |  |  | 0.648 |
| ns | 8215 |  | 87 | CI gate commands | 6.6 |  | 0.645 |
| walker |  | 8267 | 62 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 19, sub: 0, line: 957 } |  |  | 0.645 |
| walker |  | 8340 | 73 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 20, sub: 0, line: 1027 } |  |  | 0.645 |
| ns | 8369 |  | 154 | `test/util.py` complete shared-helper roster | 6.7 |  | 0.642 |
| walker |  | 8473 | 133 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 17, sub: 0, line: 763 } |  |  | 0.642 |
| ns | 8541 |  | 172 | `pyproject.toml`: build backend, version source, tooling config | 6.8 |  | 0.633 |
| walker |  | 8697 | 224 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 18, sub: 0, line: 821 } |  |  | 0.648 |
| ns | 8728 |  | 187 | `test_chronos.py` complete test-function roster | 6.9 |  | 0.641 |
| ns | 8989 |  | 261 | `test_chronos_bolt.py` complete test-function roster | 6.10 |  | 0.634 |
| walker |  | 9039 | 342 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/pipeline.py, decl: 10, sub: 0, line: 96 } |  |  | 0.653 |
| walker |  | 9074 | 35 | Code::CodeKey { rung: Doc, file: src/chronos/chronos2/pipeline.py, decl: 23, sub: 0, line: 1224 } |  |  | 0.653 |
| walker |  | 9167 | 93 | Code::CodeKey { rung: Names, file: src/chronos/chronos2/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 9212 | 45 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 2, sub: 0, line: 38 } |  |  | 0.655 |
| walker |  | 9269 | 57 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 1, sub: 0, line: 31 } |  |  | 0.656 |
| ns | 9314 |  | 325 | `test_chronos2.py` test roster, part 1: loading, predict, embed, predict_df | 6.11 |  | 0.647 |
| walker |  | 9338 | 69 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 4, sub: 0, line: 48 } |  |  | 0.647 |
| walker |  | 9408 | 70 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 5, sub: 0, line: 82 } |  |  | 0.648 |
| walker |  | 9492 | 84 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 11, sub: 0, line: 190 } |  |  | 0.648 |
| ns | 9537 |  | 223 | `test_chronos2.py` test roster, part 2: cross-learning and fine-tuning | 6.12 | 6.11 | 0.643 |
| walker |  | 9610 | 118 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 6, sub: 0, line: 89 } |  |  | 0.648 |
| walker |  | 9625 | 15 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 8, sub: 0, line: 98 } |  |  | 0.648 |
| walker |  | 9646 | 21 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 9, sub: 0, line: 112 } |  |  | 0.648 |
| walker |  | 9723 | 77 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 10, sub: 0, line: 134 } |  |  | 0.648 |
| ns | 9864 |  | 327 | `test_df_utils.py` and `test_utils.py` complete test rosters | 6.13 |  | 0.640 |
| ns | 9952 |  | 88 | Opt-in model-evaluation workflow | 6.14 |  | 0.639 |
| ns | 9978 |  | 26 | Licensing statement | 6.15 |  | 0.638 |
| walker |  | 9984 | 261 | Code::CodeKey { rung: Decl, file: src/chronos/chronos2/model.py, decl: 12, sub: 0, line: 198 } |  |  | 0.657 |

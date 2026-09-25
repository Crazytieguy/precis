Score(3000)=0.743 I=0.896 C=0.616 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.762/0.803/0.787/0.743/0.663/0.593/0.598

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 26 | 26 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 30 | 4 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 33 |  | 33 | Project name and one-line description | 1.1 |  | 0.000 |
| walker |  | 62 | 32 | Fs::DirListing { dir: nanovllm } |  |  | 0.000 |
| walker |  | 68 | 6 | Fs::DirListing { dir: nanovllm/models } |  |  | 0.000 |
| walker |  | 76 | 8 | Fs::DirListing { dir: nanovllm/utils } |  |  | 0.000 |
| ns | 89 |  | 56 | All README H2 section headings | 1.2 |  | 0.000 |
| walker |  | 109 | 33 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.577 |
| ns | 115 |  | 26 | Complete repository root listing | 1.3 |  | 0.730 |
| walker |  | 133 | 24 | Fs::DirListing { dir: nanovllm/engine } |  |  | 0.738 |
| ns | 147 |  | 32 | Complete nanovllm/ package listing | 1.4 |  | 0.759 |
| ns | 225 |  | 78 | The complete public API: __init__.py and llm.py | 1.5 |  | 0.651 |
| walker |  | 227 | 94 | Toml::Identity { file: pyproject.toml } |  |  | 0.652 |
| walker |  | 258 | 31 | Fs::DirListing { dir: nanovllm/layers } |  |  | 0.664 |
| ns | 302 |  | 77 | README Key Features bullets | 1.6 |  | 0.617 |
| walker |  | 314 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.817 |
| walker |  | 391 | 77 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.892 |
| walker |  | 430 | 39 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.892 |
| ns | 450 |  | 148 | README Quick Start usage snippet | 1.7 |  | 0.778 |
| walker |  | 505 | 75 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.781 |
| walker |  | 516 | 11 | Code::CodeKey { rung: Names, file: bench.py, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 527 | 11 | Code::CodeKey { rung: Names, file: example.py, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 558 | 31 | Code::CodeKey { rung: Names, file: nanovllm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| ns | 567 |  | 117 | SamplingParams in full | 1.8 |  | 0.698 |
| walker |  | 650 | 92 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.700 |
| walker |  | 661 | 11 | Code::CodeKey { rung: Names, file: nanovllm/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 781 |  | 214 | Config dataclass — every field with its default | 1.9 |  | 0.592 |
| walker |  | 845 | 184 | Code::CodeKey { rung: Decl, file: nanovllm/config.py, decl: 1, sub: 0, line: 6 } |  |  | 0.701 |
| walker |  | 857 | 12 | Code::CodeKey { rung: Names, file: nanovllm/sampling_params.py, decl: 0, sub: 0, line: 0 } |  |  | 0.702 |
| ns | 899 |  | 118 | Config.__post_init__ validation and derivation | 1.10 |  | 0.666 |
| walker |  | 922 | 65 | Code::CodeKey { rung: Decl, file: nanovllm/sampling_params.py, decl: 1, sub: 0, line: 4 } |  |  | 0.702 |
| ns | 923 |  | 24 | Complete nanovllm/engine/ listing | 2.1 |  | 0.714 |
| walker |  | 943 | 21 | Code::CodeKey { rung: Body, file: nanovllm/sampling_params.py, decl: 2, sub: 0, line: 10 } |  |  | 0.740 |
| ns | 972 |  | 49 | Listings for layers/, models/, utils/ and assets/ | 2.2 |  | 0.757 |
| ns | 1047 |  | 75 | LLMEngine method roster | 2.3 |  | 0.726 |
| walker |  | 1096 | 153 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.795 |
| walker |  | 1112 | 16 | Code::CodeKey { rung: Names, file: nanovllm/llm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.802 |
| walker |  | 1117 | 5 | Code::CodeKey { rung: Decl, file: nanovllm/llm.py, decl: 1, sub: 0, line: 4 } |  |  | 0.809 |
| ns | 1121 |  | 74 | Scheduler method roster | 2.4 |  | 0.778 |
| walker |  | 1138 | 21 | Code::CodeKey { rung: Names, file: nanovllm/engine/block_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| walker |  | 1189 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 1, sub: 0, line: 8 } |  |  | 0.780 |
| walker |  | 1209 | 20 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 3, sub: 0, line: 16 } |  |  | 0.780 |
| ns | 1280 |  | 159 | Block and BlockManager method roster | 2.5 |  | 0.730 |
| walker |  | 1388 | 179 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 5, sub: 0, line: 26 } |  |  | 0.793 |
| walker |  | 1396 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.803 |
| walker |  | 1411 | 15 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 10, sub: 0, line: 56 } |  |  | 0.803 |
| walker |  | 1434 | 23 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 13, sub: 0, line: 93 } |  |  | 0.803 |
| walker |  | 1456 | 22 | Code::CodeKey { rung: Names, file: nanovllm/engine/sequence.py, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| walker |  | 1484 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 1, sub: 0, line: 8 } |  |  | 0.803 |
| ns | 1569 |  | 289 | SequenceStatus members and the complete Sequence member roster | 2.6 |  | 0.717 |
| walker |  | 1710 | 226 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 2, sub: 0, line: 14 } |  |  | 0.789 |
| walker |  | 1718 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.795 |
| walker |  | 1726 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.801 |
| walker |  | 1734 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.807 |
| walker |  | 1742 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.814 |
| walker |  | 1750 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.820 |
| walker |  | 1758 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.827 |
| walker |  | 1766 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.834 |
| ns | 1772 |  | 203 | ModelRunner method roster | 2.7 |  | 0.782 |
| walker |  | 1774 | 8 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 4, sub: 0, line: 31 } |  |  | 0.782 |
| walker |  | 1785 | 11 | Code::CodeKey { rung: Names, file: nanovllm/engine/scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| walker |  | 1897 | 112 | Code::CodeKey { rung: Decl, file: nanovllm/engine/scheduler.py, decl: 1, sub: 0, line: 8 } |  |  | 0.810 |
| walker |  | 1906 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 4, sub: 0, line: 21 } |  |  | 0.810 |
| walker |  | 1919 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 3, sub: 0, line: 18 } |  |  | 0.810 |
| walker |  | 1955 | 36 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 6, sub: 0, line: 60 } |  |  | 0.810 |
| ns | 2006 |  | 234 | Complete top-level symbol roster for nanovllm/layers/ | 2.8 |  | 0.760 |
| walker |  | 2061 | 106 | Code::CodeKey { rung: Body, file: nanovllm/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.787 |
| walker |  | 2073 | 12 | Code::CodeKey { rung: Names, file: nanovllm/engine/model_runner.py, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 2176 |  | 170 | qwen3.py class roster and packed_modules_mapping | 2.9 |  | 0.760 |
| walker |  | 2352 | 279 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 1, sub: 0, line: 15 } |  |  | 0.804 |
| walker |  | 2363 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 14, sub: 0, line: 189 } |  |  | 0.811 |
| walker |  | 2374 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 16, sub: 0, line: 216 } |  |  | 0.819 |
| ns | 2388 |  | 212 | The Context dataclass and the utils/ function roster | 2.10 |  | 0.781 |
| walker |  | 2427 | 53 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 4, sub: 0, line: 61 } |  |  | 0.781 |
| walker |  | 2484 | 57 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 7, sub: 0, line: 85 } |  |  | 0.781 |
| walker |  | 2547 | 63 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 13, sub: 0, line: 182 } |  |  | 0.781 |
| ns | 2583 |  | 195 | Import blocks of llm_engine.py and scheduler.py | 2.11 |  | 0.748 |
| walker |  | 2584 | 37 | Code::CodeKey { rung: Names, file: nanovllm/layers/rotary_embedding.py, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 2608 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 2, sub: 0, line: 17 } |  |  | 0.750 |
| walker |  | 2647 | 39 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.750 |
| walker |  | 2703 | 56 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 3, sub: 0, line: 19 } |  |  | 0.750 |
| walker |  | 2764 | 61 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.750 |
| ns | 2778 |  | 195 | Import blocks of model_runner.py and block_manager.py | 2.12 |  | 0.720 |
| walker |  | 2834 | 70 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.721 |
| walker |  | 2847 | 13 | Code::CodeKey { rung: Names, file: nanovllm/engine/llm_engine.py, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 2937 | 90 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 1, sub: 0, line: 15 } |  |  | 0.742 |
| walker |  | 2946 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 6, sub: 0, line: 56 } |  |  | 0.742 |
| walker |  | 3008 | 62 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 7, sub: 0, line: 59 } |  |  | 0.743 |
| ns | 3012 |  | 234 | Import blocks of qwen3.py and attention.py | 2.13 |  | 0.714 |
| walker |  | 3047 | 39 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 3, sub: 0, line: 36 } |  |  | 0.714 |
| ns | 3203 |  | 191 | pyproject project metadata and dependency list | 2.14 |  | 0.716 |
| ns | 3324 |  | 121 | README installation and model-download commands | 2.15 |  | 0.722 |
| walker |  | 3326 | 279 | Code::CodeKey { rung: Body, file: example.py, decl: 1, sub: 0, line: 6 } |  |  | 0.722 |
| walker |  | 3340 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/layernorm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 3388 | 48 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 1, sub: 0, line: 5 } |  |  | 0.723 |
| walker |  | 3424 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.723 |
| ns | 3458 |  | 134 | LLMEngine.step and is_finished | 3.1 | 2.3 | 0.710 |
| walker |  | 3465 | 41 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.710 |
| walker |  | 3514 | 49 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.710 |
| walker |  | 3565 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.710 |
| walker |  | 3579 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/sampler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| ns | 3604 |  | 146 | Scheduler construction and queue predicates | 3.2 | 2.4 | 0.698 |
| walker |  | 3616 | 37 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 1, sub: 0, line: 5 } |  |  | 0.698 |
| walker |  | 3625 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.698 |
| walker |  | 3632 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 2, sub: 0, line: 7 } |  |  | 0.698 |
| ns | 3841 |  | 237 | Scheduler.schedule — the prefill pass | 3.3 | 2.4 | 0.675 |
| walker |  | 3854 | 222 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.676 |
| walker |  | 3954 | 100 | Code::CodeKey { rung: Names, file: nanovllm/layers/linear.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3988 | 34 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 2, sub: 0, line: 12 } |  |  | 0.691 |
| walker |  | 4032 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 13, sub: 0, line: 76 } |  |  | 0.691 |
| ns | 4064 |  | 223 | Scheduler.schedule — the decode pass and preempt | 3.4 | 2.4 | 0.669 |
| walker |  | 4076 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 16, sub: 0, line: 96 } |  |  | 0.669 |
| walker |  | 4121 | 45 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.669 |
| ns | 4176 |  | 112 | Scheduler.postprocess — the stop condition | 3.5 | 2.4 | 0.662 |
| walker |  | 4178 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 5, sub: 0, line: 37 } |  |  | 0.662 |
| walker |  | 4222 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.662 |
| walker |  | 4279 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 9, sub: 0, line: 54 } |  |  | 0.663 |
| walker |  | 4323 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 10, sub: 0, line: 56 } |  |  | 0.663 |
| ns | 4335 |  | 159 | Sequence.__init__ — per-request state | 3.6 | 2.6 | 0.651 |
| walker |  | 4380 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 19, sub: 0, line: 131 } |  |  | 0.651 |
| walker |  | 4424 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 20, sub: 0, line: 133 } |  |  | 0.651 |
| walker |  | 4482 | 58 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 3, sub: 0, line: 14 } |  |  | 0.651 |
| ns | 4518 |  | 183 | LLMEngine.generate signature and request submission | 3.7 | 2.3 | 0.641 |
| walker |  | 4554 | 72 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 17, sub: 0, line: 98 } |  |  | 0.641 |
| walker |  | 4599 | 45 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 4, sub: 0, line: 42 } |  |  | 0.642 |
| walker |  | 4632 | 33 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.642 |
| walker |  | 4641 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 5, sub: 0, line: 34 } |  |  | 0.642 |
| walker |  | 4718 | 77 | Code::CodeKey { rung: Names, file: nanovllm/models/qwen3.py, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 4742 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 1, sub: 0, line: 14 } |  |  | 0.645 |
| walker |  | 4766 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 7, sub: 0, line: 119 } |  |  | 0.645 |
| walker |  | 4790 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 10, sub: 0, line: 161 } |  |  | 0.645 |
| ns | 4810 |  | 292 | LLMEngine.generate — drive loop and output assembly | 3.8 | 3.7 | 0.626 |
| walker |  | 4817 | 27 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 4, sub: 0, line: 90 } |  |  | 0.626 |
| walker |  | 4845 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 8, sub: 0, line: 121 } |  |  | 0.626 |
| walker |  | 4873 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 11, sub: 0, line: 163 } |  |  | 0.626 |
| walker |  | 4911 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 3, sub: 0, line: 71 } |  |  | 0.627 |
| walker |  | 4949 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 12, sub: 0, line: 172 } |  |  | 0.627 |
| walker |  | 4995 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 5, sub: 0, line: 92 } |  |  | 0.627 |
| walker |  | 5050 | 55 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 9, sub: 0, line: 145 } |  |  | 0.627 |
| ns | 5059 |  | 249 | LLMEngine.__init__ — config filtering and TP worker spawn | 3.9 | 2.3 | 0.612 |
| ns | 5163 |  | 104 | LLMEngine.exit and add_request | 3.10 | 2.3 | 0.618 |
| walker |  | 5186 | 136 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 13, sub: 0, line: 185 } |  |  | 0.637 |
| walker |  | 5214 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 14, sub: 0, line: 194 } |  |  | 0.637 |
| walker |  | 5242 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.637 |
| walker |  | 5280 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.637 |
| ns | 5345 |  | 182 | BlockManager.__init__ and compute_hash | 3.11 | 2.5 | 0.627 |
| walker |  | 5418 | 138 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 2, sub: 0, line: 16 } |  |  | 0.627 |
| walker |  | 5449 | 31 | Code::CodeKey { rung: Names, file: nanovllm/layers/embed_head.py, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 5479 | 30 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 5, sub: 0, line: 45 } |  |  | 0.633 |
| walker |  | 5523 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.633 |
| walker |  | 5576 | 53 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 1, sub: 0, line: 9 } |  |  | 0.633 |
| walker |  | 5609 | 33 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.633 |
| walker |  | 5631 | 22 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.633 |
| walker |  | 5647 | 16 | Code::CodeKey { rung: Names, file: nanovllm/layers/activation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 5683 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 1, sub: 0, line: 6 } |  |  | 0.635 |
| ns | 5686 |  | 341 | BlockManager.can_allocate and allocate — the prefix-cache hit path | 3.12 | 2.5 | 0.615 |
| walker |  | 5692 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.615 |
| walker |  | 5699 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 2, sub: 0, line: 8 } |  |  | 0.615 |
| walker |  | 5728 | 29 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.615 |
| walker |  | 5807 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.616 |
| ns | 6057 |  | 371 | BlockManager.deallocate, can_append and may_append | 3.13 | 2.5 | 0.596 |
| walker |  | 6173 | 366 | Code::CodeKey { rung: Body, file: bench.py, decl: 1, sub: 0, line: 8 } |  |  | 0.596 |
| ns | 6182 |  | 125 | Block class body | 3.14 | 2.5 | 0.590 |
| walker |  | 6213 | 40 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.590 |
| walker |  | 6244 | 31 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 4, sub: 0, line: 20 } |  |  | 0.595 |
| walker |  | 6252 | 8 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 4, sub: 0, line: 33 } |  |  | 0.595 |
| ns | 6344 |  | 162 | Sequence block arithmetic and append_token | 3.15 | 2.6 | 0.585 |
| walker |  | 6354 | 102 | Code::CodeKey { rung: Names, file: nanovllm/utils/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 6362 | 8 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 3, sub: 0, line: 18 } |  |  | 0.587 |
| walker |  | 6381 | 19 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 5, sub: 0, line: 25 } |  |  | 0.587 |
| ns | 6469 |  | 125 | Sequence.__getstate__ / __setstate__ — the TP pickling contract | 3.16 | 2.6 | 0.581 |
| walker |  | 6514 | 133 | Code::CodeKey { rung: Decl, file: nanovllm/utils/context.py, decl: 1, sub: 0, line: 5 } |  |  | 0.599 |
| walker |  | 6570 | 56 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 4, sub: 0, line: 21 } |  |  | 0.599 |
| walker |  | 6611 | 41 | Code::CodeKey { rung: Names, file: nanovllm/utils/loader.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 6621 | 10 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 1, sub: 0, line: 8 } |  |  | 0.603 |
| walker |  | 6686 | 65 | Code::CodeKey { rung: Names, file: nanovllm/layers/attention.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6726 | 40 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 3, sub: 0, line: 43 } |  |  | 0.611 |
| walker |  | 6772 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.611 |
| walker |  | 6857 | 85 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.614 |
| ns | 6881 |  | 412 | ModelRunner.run and run_model — the per-step dispatch | 4.1 | 2.7 | 0.599 |
| walker |  | 6930 | 73 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.599 |
| walker |  | 7017 | 87 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 10, sub: 0, line: 120 } |  |  | 0.599 |
| walker |  | 7029 | 12 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.599 |
| walker |  | 7071 | 42 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.599 |
| walker |  | 7081 | 10 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 7, sub: 0, line: 47 } |  |  | 0.599 |
| walker |  | 7174 | 93 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 7, sub: 0, line: 65 } |  |  | 0.608 |
| walker |  | 7265 | 91 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 5, sub: 0, line: 68 } |  |  | 0.608 |
| walker |  | 7278 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.608 |
| ns | 7295 |  | 414 | ModelRunner.__init__ — distributed init and startup order | 4.2 | 2.7 | 0.590 |
| walker |  | 7349 | 71 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 3, sub: 0, line: 27 } |  |  | 0.590 |
| walker |  | 7391 | 42 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 9, sub: 0, line: 51 } |  |  | 0.590 |
| walker |  | 7404 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.590 |
| walker |  | 7503 | 99 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.590 |
| walker |  | 7514 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.590 |
| walker |  | 7527 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.590 |
| walker |  | 7570 | 43 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 2, sub: 0, line: 10 } |  |  | 0.598 |
| ns | 7645 |  | 350 | ModelRunner.allocate_kv_cache — sizing the paged cache | 4.3 | 2.7 | 0.588 |
| walker |  | 7673 | 103 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.600 |
| walker |  | 7687 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 8, sub: 0, line: 50 } |  |  | 0.600 |
| walker |  | 7914 | 227 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 2, sub: 0, line: 12 } |  |  | 0.600 |
| walker |  | 7993 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.600 |
| ns | 8059 |  | 414 | ModelRunner.prepare_prefill — varlen batching and slot mapping | 4.4 | 2.7 | 0.586 |
| walker |  | 8153 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.586 |
| walker |  | 8164 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.586 |
| walker |  | 8177 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.588 |
| ns | 8235 |  | 176 | ModelRunner.prepare_decode — one token per sequence | 4.5 | 2.7 | 0.581 |
| walker |  | 8295 | 118 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 5, sub: 0, line: 48 } |  |  | 0.591 |
| walker |  | 8309 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 12, sub: 0, line: 72 } |  |  | 0.591 |
| walker |  | 8413 | 104 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 6, sub: 0, line: 76 } |  |  | 0.591 |
| walker |  | 8493 | 80 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.591 |
| ns | 8552 |  | 317 | ModelRunner.capture_cudagraph — the graph ladder | 4.6 | 2.7 | 0.580 |
| walker |  | 8575 | 82 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.580 |
| walker |  | 8590 | 15 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.580 |
| ns | 8701 |  | 149 | Complete method roster for layers/linear.py | 5.1 | 2.8 | 0.588 |
| walker |  | 8705 | 115 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 4, sub: 0, line: 34 } |  |  | 0.588 |
| ns | 8810 |  | 109 | Complete method roster for models/qwen3.py | 5.2 | 2.9 | 0.593 |
| walker |  | 8866 | 161 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 2, sub: 0, line: 33 } |  |  | 0.594 |
| walker |  | 8977 | 111 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 15, sub: 0, line: 208 } |  |  | 0.597 |
| walker |  | 9041 | 64 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.601 |
| walker |  | 9062 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.603 |
| ns | 9141 |  | 331 | Attention.forward — paged KV write and flash-attn dispatch | 5.3 | 2.8 | 0.595 |
| walker |  | 9186 | 124 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 8, sub: 0, line: 91 } |  |  | 0.595 |
| walker |  | 9288 | 102 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.595 |
| walker |  | 9309 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.597 |
| walker |  | 9330 | 21 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 1, sub: 0, line: 7 } |  |  | 0.597 |
| ns | 9387 |  | 246 | Qwen3Attention.forward — QKV split, q/k norm, RoPE | 5.4 | 5.2 | 0.590 |
| walker |  | 9400 | 70 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 8, sub: 0, line: 43 } |  |  | 0.590 |
| walker |  | 9560 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.590 |
| ns | 9575 |  | 188 | store_kvcache and its Triton kernel boundary | 5.5 | 2.8 | 0.594 |
| ns | 9684 |  | 109 | Sampler.forward — the exponential-noise argmax trick | 5.6 | 2.8 | 0.597 |
| walker |  | 9803 | 243 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 2, sub: 0, line: 17 } |  |  | 0.615 |
| walker |  | 9831 | 28 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.615 |
| ns | 9865 |  | 181 | ParallelLMHead.forward — last-token selection and logit gather | 5.7 | 2.8 | 0.610 |
| ns | 9974 |  | 109 | README benchmark pointer and throughput table | 6.1 |  | 0.612 |
| walker |  | 9996 | 165 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 7, sub: 0, line: 56 } |  |  | 0.622 |

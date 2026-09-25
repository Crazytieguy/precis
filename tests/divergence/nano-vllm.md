Score(3000)=0.613 I=0.818 C=0.460 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.670/0.626/0.577/0.613/0.646/0.609/0.603

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
| walker |  | 120 | 11 | Code::CodeKey { rung: Names, file: bench.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| walker |  | 131 | 11 | Code::CodeKey { rung: Names, file: example.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| ns | 147 |  | 32 | Complete nanovllm/ package listing | 1.4 |  | 0.751 |
| walker |  | 155 | 24 | Fs::DirListing { dir: nanovllm/engine } |  |  | 0.759 |
| ns | 225 |  | 78 | The complete public API: __init__.py and llm.py | 1.5 |  | 0.651 |
| walker |  | 249 | 94 | Toml::Identity { file: pyproject.toml } |  |  | 0.652 |
| walker |  | 280 | 31 | Fs::DirListing { dir: nanovllm/layers } |  |  | 0.664 |
| ns | 302 |  | 77 | README Key Features bullets | 1.6 |  | 0.617 |
| walker |  | 311 | 31 | Code::CodeKey { rung: Names, file: nanovllm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 333 | 22 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.638 |
| walker |  | 344 | 11 | Code::CodeKey { rung: Names, file: nanovllm/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 400 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.838 |
| ns | 450 |  | 148 | README Quick Start usage snippet | 1.7 |  | 0.730 |
| walker |  | 477 | 77 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.795 |
| walker |  | 552 | 75 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.799 |
| walker |  | 564 | 12 | Code::CodeKey { rung: Names, file: nanovllm/sampling_params.py, decl: 0, sub: 0, line: 0 } |  |  | 0.799 |
| ns | 567 |  | 117 | SamplingParams in full | 1.8 |  | 0.700 |
| walker |  | 580 | 16 | Code::CodeKey { rung: Names, file: nanovllm/llm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 585 | 5 | Code::CodeKey { rung: Decl, file: nanovllm/llm.py, decl: 1, sub: 0, line: 4 } |  |  | 0.725 |
| walker |  | 650 | 65 | Code::CodeKey { rung: Decl, file: nanovllm/sampling_params.py, decl: 1, sub: 0, line: 4 } |  |  | 0.774 |
| walker |  | 661 | 11 | Code::CodeKey { rung: Names, file: nanovllm/engine/scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| walker |  | 673 | 12 | Code::CodeKey { rung: Names, file: nanovllm/engine/model_runner.py, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| walker |  | 686 | 13 | Code::CodeKey { rung: Names, file: nanovllm/engine/llm_engine.py, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| walker |  | 700 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/layernorm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| walker |  | 714 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/sampler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| walker |  | 751 | 37 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 1, sub: 0, line: 5 } |  |  | 0.774 |
| walker |  | 760 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.774 |
| ns | 781 |  | 214 | Config dataclass — every field with its default | 1.9 |  | 0.655 |
| walker |  | 808 | 48 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 1, sub: 0, line: 5 } |  |  | 0.655 |
| walker |  | 847 | 39 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.655 |
| walker |  | 863 | 16 | Code::CodeKey { rung: Names, file: nanovllm/layers/activation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 899 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 1, sub: 0, line: 6 } |  |  | 0.620 |
| ns | 899 |  | 118 | Config.__post_init__ validation and derivation | 1.10 |  | 0.620 |
| walker |  | 908 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.620 |
| ns | 923 |  | 24 | Complete nanovllm/engine/ listing | 2.1 |  | 0.639 |
| ns | 972 |  | 49 | Listings for layers/, models/, utils/ and assets/ | 2.2 |  | 0.670 |
| walker |  | 1004 | 96 | Toml::Config { file: pyproject.toml } |  |  | 0.670 |
| walker |  | 1025 | 21 | Code::CodeKey { rung: Names, file: nanovllm/engine/block_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| ns | 1047 |  | 75 | LLMEngine method roster | 2.3 |  | 0.644 |
| walker |  | 1076 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 1, sub: 0, line: 8 } |  |  | 0.645 |
| walker |  | 1112 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.645 |
| ns | 1121 |  | 74 | Scheduler method roster | 2.4 |  | 0.621 |
| walker |  | 1134 | 22 | Code::CodeKey { rung: Names, file: nanovllm/engine/sequence.py, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 1162 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 1, sub: 0, line: 8 } |  |  | 0.622 |
| walker |  | 1203 | 41 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.622 |
| walker |  | 1210 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 2, sub: 0, line: 8 } |  |  | 0.622 |
| walker |  | 1217 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 2, sub: 0, line: 7 } |  |  | 0.622 |
| ns | 1280 |  | 159 | Block and BlockManager method roster | 2.5 |  | 0.584 |
| walker |  | 1307 | 90 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 1, sub: 0, line: 15 } |  |  | 0.626 |
| walker |  | 1356 | 49 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.626 |
| walker |  | 1540 | 184 | Code::CodeKey { rung: Decl, file: nanovllm/config.py, decl: 1, sub: 0, line: 6 } |  |  | 0.691 |
| ns | 1569 |  | 289 | SequenceStatus members and the complete Sequence member roster | 2.6 |  | 0.618 |
| walker |  | 1591 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.618 |
| walker |  | 1622 | 31 | Code::CodeKey { rung: Names, file: nanovllm/layers/embed_head.py, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 1652 | 30 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 5, sub: 0, line: 45 } |  |  | 0.618 |
| walker |  | 1705 | 53 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 1, sub: 0, line: 9 } |  |  | 0.618 |
| walker |  | 1738 | 33 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.618 |
| ns | 1772 |  | 203 | ModelRunner method roster | 2.7 |  | 0.579 |
| walker |  | 1782 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.579 |
| walker |  | 1791 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 6, sub: 0, line: 56 } |  |  | 0.580 |
| walker |  | 1903 | 112 | Code::CodeKey { rung: Decl, file: nanovllm/engine/scheduler.py, decl: 1, sub: 0, line: 8 } |  |  | 0.611 |
| walker |  | 1912 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 4, sub: 0, line: 21 } |  |  | 0.611 |
| walker |  | 1974 | 62 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 7, sub: 0, line: 59 } |  |  | 0.611 |
| ns | 2006 |  | 234 | Complete top-level symbol roster for nanovllm/layers/ | 2.8 |  | 0.577 |
| ns | 2176 |  | 170 | qwen3.py class roster and packed_modules_mapping | 2.9 |  | 0.557 |
| walker |  | 2253 | 279 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 1, sub: 0, line: 15 } |  |  | 0.608 |
| walker |  | 2264 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 14, sub: 0, line: 189 } |  |  | 0.616 |
| walker |  | 2275 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 16, sub: 0, line: 216 } |  |  | 0.624 |
| walker |  | 2312 | 37 | Code::CodeKey { rung: Names, file: nanovllm/layers/rotary_embedding.py, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2336 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 2, sub: 0, line: 17 } |  |  | 0.631 |
| walker |  | 2375 | 39 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.631 |
| ns | 2388 |  | 212 | The Context dataclass and the utils/ function roster | 2.10 |  | 0.602 |
| walker |  | 2445 | 70 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.604 |
| walker |  | 2501 | 56 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 3, sub: 0, line: 19 } |  |  | 0.604 |
| walker |  | 2562 | 61 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.604 |
| ns | 2583 |  | 195 | Import blocks of llm_engine.py and scheduler.py | 2.11 |  | 0.579 |
| walker |  | 2618 | 56 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.579 |
| walker |  | 2639 | 21 | Code::CodeKey { rung: Body, file: nanovllm/sampling_params.py, decl: 2, sub: 0, line: 10 } |  |  | 0.590 |
| walker |  | 2680 | 41 | Code::CodeKey { rung: Names, file: nanovllm/utils/loader.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 2690 | 10 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 1, sub: 0, line: 8 } |  |  | 0.591 |
| walker |  | 2703 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 3, sub: 0, line: 18 } |  |  | 0.591 |
| ns | 2778 |  | 195 | Import blocks of model_runner.py and block_manager.py | 2.12 |  | 0.568 |
| walker |  | 2795 | 92 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.570 |
| walker |  | 2974 | 179 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 5, sub: 0, line: 26 } |  |  | 0.607 |
| walker |  | 2982 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.613 |
| walker |  | 2997 | 15 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 10, sub: 0, line: 56 } |  |  | 0.613 |
| ns | 3012 |  | 234 | Import blocks of qwen3.py and attention.py | 2.13 |  | 0.589 |
| walker |  | 3062 | 65 | Code::CodeKey { rung: Names, file: nanovllm/layers/attention.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 3102 | 40 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 3, sub: 0, line: 43 } |  |  | 0.598 |
| walker |  | 3187 | 85 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.601 |
| ns | 3203 |  | 191 | pyproject project metadata and dependency list | 2.14 |  | 0.618 |
| walker |  | 3233 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.618 |
| ns | 3324 |  | 121 | README installation and model-download commands | 2.15 |  | 0.627 |
| ns | 3458 |  | 134 | LLMEngine.step and is_finished | 3.1 | 2.3 | 0.616 |
| walker |  | 3459 | 226 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 2, sub: 0, line: 14 } |  |  | 0.658 |
| walker |  | 3467 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.661 |
| walker |  | 3475 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.664 |
| walker |  | 3483 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.668 |
| walker |  | 3491 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.672 |
| walker |  | 3499 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.676 |
| walker |  | 3507 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.680 |
| walker |  | 3515 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.684 |
| walker |  | 3592 | 77 | Code::CodeKey { rung: Names, file: nanovllm/models/qwen3.py, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| ns | 3604 |  | 146 | Scheduler construction and queue predicates | 3.2 | 2.4 | 0.675 |
| walker |  | 3616 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 1, sub: 0, line: 14 } |  |  | 0.675 |
| walker |  | 3640 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 7, sub: 0, line: 119 } |  |  | 0.675 |
| walker |  | 3664 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 10, sub: 0, line: 161 } |  |  | 0.675 |
| walker |  | 3691 | 27 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 4, sub: 0, line: 90 } |  |  | 0.675 |
| walker |  | 3719 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 8, sub: 0, line: 121 } |  |  | 0.675 |
| walker |  | 3747 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 11, sub: 0, line: 163 } |  |  | 0.675 |
| walker |  | 3785 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 3, sub: 0, line: 71 } |  |  | 0.675 |
| walker |  | 3823 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 12, sub: 0, line: 172 } |  |  | 0.675 |
| ns | 3841 |  | 237 | Scheduler.schedule — the prefill pass | 3.3 | 2.4 | 0.653 |
| walker |  | 3869 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 5, sub: 0, line: 92 } |  |  | 0.653 |
| walker |  | 3924 | 55 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 9, sub: 0, line: 145 } |  |  | 0.653 |
| walker |  | 4060 | 136 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 13, sub: 0, line: 185 } |  |  | 0.676 |
| ns | 4064 |  | 223 | Scheduler.schedule — the decode pass and preempt | 3.4 | 2.4 | 0.653 |
| walker |  | 4088 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 14, sub: 0, line: 194 } |  |  | 0.653 |
| walker |  | 4116 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.653 |
| walker |  | 4154 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.653 |
| ns | 4176 |  | 112 | Scheduler.postprocess — the stop condition | 3.5 | 2.4 | 0.646 |
| walker |  | 4207 | 53 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 4, sub: 0, line: 61 } |  |  | 0.646 |
| ns | 4335 |  | 159 | Sequence.__init__ — per-request state | 3.6 | 2.6 | 0.635 |
| walker |  | 4345 | 138 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 2, sub: 0, line: 16 } |  |  | 0.635 |
| walker |  | 4445 | 100 | Code::CodeKey { rung: Names, file: nanovllm/layers/linear.py, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 4479 | 34 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 2, sub: 0, line: 12 } |  |  | 0.659 |
| ns | 4518 |  | 183 | LLMEngine.generate signature and request submission | 3.7 | 2.3 | 0.649 |
| walker |  | 4523 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 13, sub: 0, line: 76 } |  |  | 0.649 |
| walker |  | 4567 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 16, sub: 0, line: 96 } |  |  | 0.649 |
| walker |  | 4624 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 5, sub: 0, line: 37 } |  |  | 0.649 |
| walker |  | 4681 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 9, sub: 0, line: 54 } |  |  | 0.649 |
| walker |  | 4738 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 19, sub: 0, line: 131 } |  |  | 0.650 |
| walker |  | 4782 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.650 |
| ns | 4810 |  | 292 | LLMEngine.generate — drive loop and output assembly | 3.8 | 3.7 | 0.631 |
| walker |  | 4826 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 10, sub: 0, line: 56 } |  |  | 0.631 |
| walker |  | 4870 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 20, sub: 0, line: 133 } |  |  | 0.631 |
| walker |  | 4915 | 45 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.631 |
| walker |  | 4973 | 58 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 3, sub: 0, line: 14 } |  |  | 0.631 |
| walker |  | 5045 | 72 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 17, sub: 0, line: 98 } |  |  | 0.631 |
| ns | 5059 |  | 249 | LLMEngine.__init__ — config filtering and TP worker spawn | 3.9 | 2.3 | 0.616 |
| walker |  | 5147 | 102 | Code::CodeKey { rung: Names, file: nanovllm/utils/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 5155 | 8 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 3, sub: 0, line: 18 } |  |  | 0.619 |
| ns | 5163 |  | 104 | LLMEngine.exit and add_request | 3.10 | 2.3 | 0.611 |
| walker |  | 5174 | 19 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 5, sub: 0, line: 25 } |  |  | 0.611 |
| walker |  | 5307 | 133 | Code::CodeKey { rung: Decl, file: nanovllm/utils/context.py, decl: 1, sub: 0, line: 5 } |  |  | 0.635 |
| walker |  | 5336 | 29 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.635 |
| ns | 5345 |  | 182 | BlockManager.__init__ and compute_hash | 3.11 | 2.5 | 0.625 |
| walker |  | 5392 | 56 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 4, sub: 0, line: 21 } |  |  | 0.625 |
| walker |  | 5412 | 20 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 3, sub: 0, line: 16 } |  |  | 0.625 |
| ns | 5686 |  | 341 | BlockManager.can_allocate and allocate — the prefix-cache hit path | 3.12 | 2.5 | 0.605 |
| walker |  | 5691 | 279 | Code::CodeKey { rung: Body, file: example.py, decl: 1, sub: 0, line: 6 } |  |  | 0.605 |
| walker |  | 5713 | 22 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.605 |
| walker |  | 5866 | 153 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.630 |
| walker |  | 5902 | 36 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 6, sub: 0, line: 60 } |  |  | 0.631 |
| walker |  | 5959 | 57 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 7, sub: 0, line: 85 } |  |  | 0.631 |
| walker |  | 5998 | 39 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 3, sub: 0, line: 36 } |  |  | 0.635 |
| ns | 6057 |  | 371 | BlockManager.deallocate, can_append and may_append | 3.13 | 2.5 | 0.615 |
| ns | 6182 |  | 125 | Block class body | 3.14 | 2.5 | 0.609 |
| ns | 6344 |  | 162 | Sequence block arithmetic and append_token | 3.15 | 2.6 | 0.598 |
| walker |  | 6364 | 366 | Code::CodeKey { rung: Body, file: bench.py, decl: 1, sub: 0, line: 8 } |  |  | 0.598 |
| walker |  | 6404 | 40 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.598 |
| walker |  | 6427 | 23 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 13, sub: 0, line: 93 } |  |  | 0.598 |
| walker |  | 6435 | 8 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 4, sub: 0, line: 31 } |  |  | 0.599 |
| ns | 6469 |  | 125 | Sequence.__getstate__ / __setstate__ — the TP pickling contract | 3.16 | 2.6 | 0.593 |
| walker |  | 6541 | 106 | Code::CodeKey { rung: Body, file: nanovllm/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.606 |
| walker |  | 6604 | 63 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 13, sub: 0, line: 182 } |  |  | 0.606 |
| walker |  | 6826 | 222 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.606 |
| walker |  | 6871 | 45 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 4, sub: 0, line: 42 } |  |  | 0.615 |
| ns | 6881 |  | 412 | ModelRunner.run and run_model — the per-step dispatch | 4.1 | 2.7 | 0.601 |
| walker |  | 6904 | 33 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.601 |
| walker |  | 6913 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 5, sub: 0, line: 34 } |  |  | 0.601 |
| walker |  | 6986 | 73 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.601 |
| walker |  | 7065 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.601 |
| walker |  | 7096 | 31 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 4, sub: 0, line: 20 } |  |  | 0.605 |
| walker |  | 7104 | 8 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 4, sub: 0, line: 33 } |  |  | 0.605 |
| walker |  | 7191 | 87 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 10, sub: 0, line: 120 } |  |  | 0.605 |
| walker |  | 7203 | 12 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.605 |
| ns | 7295 |  | 414 | ModelRunner.__init__ — distributed init and startup order | 4.2 | 2.7 | 0.587 |
| walker |  | 7430 | 227 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 2, sub: 0, line: 12 } |  |  | 0.587 |
| walker |  | 7472 | 42 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.587 |
| walker |  | 7552 | 80 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.587 |
| ns | 7645 |  | 350 | ModelRunner.allocate_kv_cache — sizing the paged cache | 4.3 | 2.7 | 0.577 |
| walker |  | 7712 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.577 |
| walker |  | 7722 | 10 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 7, sub: 0, line: 47 } |  |  | 0.577 |
| walker |  | 7815 | 93 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 7, sub: 0, line: 65 } |  |  | 0.586 |
| walker |  | 7906 | 91 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 5, sub: 0, line: 68 } |  |  | 0.586 |
| walker |  | 7919 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.586 |
| walker |  | 7990 | 71 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 3, sub: 0, line: 27 } |  |  | 0.586 |
| walker |  | 8033 | 43 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 2, sub: 0, line: 10 } |  |  | 0.594 |
| walker |  | 8046 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.594 |
| ns | 8059 |  | 414 | ModelRunner.prepare_prefill — varlen batching and slot mapping | 4.4 | 2.7 | 0.580 |
| walker |  | 8067 | 21 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 1, sub: 0, line: 7 } |  |  | 0.580 |
| walker |  | 8228 | 161 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 2, sub: 0, line: 33 } |  |  | 0.581 |
| ns | 8235 |  | 176 | ModelRunner.prepare_decode — one token per sequence | 4.5 | 2.7 | 0.574 |
| walker |  | 8327 | 99 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.574 |
| walker |  | 8338 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.574 |
| walker |  | 8351 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.574 |
| walker |  | 8454 | 103 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.585 |
| walker |  | 8465 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.585 |
| walker |  | 8478 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.587 |
| ns | 8552 |  | 317 | ModelRunner.capture_cudagraph — the graph ladder | 4.6 | 2.7 | 0.576 |
| walker |  | 8596 | 118 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 5, sub: 0, line: 48 } |  |  | 0.586 |
| walker |  | 8610 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 8, sub: 0, line: 50 } |  |  | 0.586 |
| ns | 8701 |  | 149 | Complete method roster for layers/linear.py | 5.1 | 2.8 | 0.594 |
| walker |  | 8714 | 104 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 6, sub: 0, line: 76 } |  |  | 0.594 |
| walker |  | 8793 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.594 |
| walker |  | 8807 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 12, sub: 0, line: 72 } |  |  | 0.594 |
| ns | 8810 |  | 109 | Complete method roster for models/qwen3.py | 5.2 | 2.9 | 0.599 |
| walker |  | 8889 | 82 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.599 |
| walker |  | 8953 | 64 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.603 |
| walker |  | 9068 | 115 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 4, sub: 0, line: 34 } |  |  | 0.603 |
| ns | 9141 |  | 331 | Attention.forward — paged KV write and flash-attn dispatch | 5.3 | 2.8 | 0.595 |
| walker |  | 9179 | 111 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 15, sub: 0, line: 208 } |  |  | 0.598 |
| walker |  | 9194 | 15 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.598 |
| walker |  | 9215 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.600 |
| walker |  | 9339 | 124 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 8, sub: 0, line: 91 } |  |  | 0.600 |
| ns | 9387 |  | 246 | Qwen3Attention.forward — QKV split, q/k norm, RoPE | 5.4 | 5.2 | 0.593 |
| walker |  | 9441 | 102 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.593 |
| walker |  | 9462 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.595 |
| walker |  | 9548 | 86 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 12, sub: 0, line: 84 } |  |  | 0.598 |
| ns | 9575 |  | 188 | store_kvcache and its Triton kernel boundary | 5.5 | 2.8 | 0.602 |
| ns | 9684 |  | 109 | Sampler.forward — the exponential-noise argmax trick | 5.6 | 2.8 | 0.605 |
| walker |  | 9708 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.605 |
| walker |  | 9796 | 88 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 6, sub: 0, line: 28 } |  |  | 0.614 |
| ns | 9865 |  | 181 | ParallelLMHead.forward — last-token selection and logit gather | 5.7 | 2.8 | 0.609 |
| ns | 9974 |  | 109 | README benchmark pointer and throughput table | 6.1 |  | 0.611 |

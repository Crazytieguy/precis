Score(3000)=0.570 I=0.799 C=0.406 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.670/0.645/0.592/0.570/0.648/0.623/0.602

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
| walker |  | 767 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 2, sub: 0, line: 7 } |  |  | 0.774 |
| ns | 781 |  | 214 | Config dataclass — every field with its default | 1.9 |  | 0.655 |
| walker |  | 815 | 48 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 1, sub: 0, line: 5 } |  |  | 0.655 |
| walker |  | 851 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.655 |
| walker |  | 892 | 41 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.655 |
| ns | 899 |  | 118 | Config.__post_init__ validation and derivation | 1.10 |  | 0.620 |
| ns | 923 |  | 24 | Complete nanovllm/engine/ listing | 2.1 |  | 0.639 |
| walker |  | 931 | 39 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.639 |
| ns | 972 |  | 49 | Listings for layers/, models/, utils/ and assets/ | 2.2 |  | 0.670 |
| walker |  | 980 | 49 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.670 |
| walker |  | 1031 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.670 |
| walker |  | 1047 | 16 | Code::CodeKey { rung: Names, file: nanovllm/layers/activation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 1047 |  | 75 | LLMEngine method roster | 2.3 |  | 0.644 |
| walker |  | 1083 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 1, sub: 0, line: 6 } |  |  | 0.644 |
| walker |  | 1092 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.644 |
| walker |  | 1099 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 2, sub: 0, line: 8 } |  |  | 0.644 |
| walker |  | 1120 | 21 | Code::CodeKey { rung: Body, file: nanovllm/sampling_params.py, decl: 2, sub: 0, line: 10 } |  |  | 0.666 |
| ns | 1121 |  | 74 | Scheduler method roster | 2.4 |  | 0.642 |
| walker |  | 1216 | 96 | Toml::Config { file: pyproject.toml } |  |  | 0.642 |
| walker |  | 1237 | 21 | Code::CodeKey { rung: Names, file: nanovllm/engine/block_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 1280 |  | 159 | Block and BlockManager method roster | 2.5 |  | 0.597 |
| walker |  | 1288 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 1, sub: 0, line: 8 } |  |  | 0.604 |
| walker |  | 1310 | 22 | Code::CodeKey { rung: Names, file: nanovllm/engine/sequence.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 1338 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 1, sub: 0, line: 8 } |  |  | 0.604 |
| walker |  | 1428 | 90 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 1, sub: 0, line: 15 } |  |  | 0.645 |
| walker |  | 1437 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 6, sub: 0, line: 56 } |  |  | 0.645 |
| walker |  | 1499 | 62 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 7, sub: 0, line: 59 } |  |  | 0.646 |
| ns | 1569 |  | 289 | SequenceStatus members and the complete Sequence member roster | 2.6 |  | 0.578 |
| walker |  | 1683 | 184 | Code::CodeKey { rung: Decl, file: nanovllm/config.py, decl: 1, sub: 0, line: 6 } |  |  | 0.635 |
| walker |  | 1714 | 31 | Code::CodeKey { rung: Names, file: nanovllm/layers/embed_head.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 1744 | 30 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 5, sub: 0, line: 45 } |  |  | 0.635 |
| ns | 1772 |  | 203 | ModelRunner method roster | 2.7 |  | 0.595 |
| walker |  | 1788 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.595 |
| walker |  | 1841 | 53 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 1, sub: 0, line: 9 } |  |  | 0.595 |
| walker |  | 1874 | 33 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.595 |
| walker |  | 1986 | 112 | Code::CodeKey { rung: Decl, file: nanovllm/engine/scheduler.py, decl: 1, sub: 0, line: 8 } |  |  | 0.626 |
| walker |  | 1995 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 4, sub: 0, line: 21 } |  |  | 0.626 |
| ns | 2006 |  | 234 | Complete top-level symbol roster for nanovllm/layers/ | 2.8 |  | 0.592 |
| walker |  | 2008 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 3, sub: 0, line: 18 } |  |  | 0.592 |
| ns | 2176 |  | 170 | qwen3.py class roster and packed_modules_mapping | 2.9 |  | 0.571 |
| walker |  | 2287 | 279 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 1, sub: 0, line: 15 } |  |  | 0.621 |
| walker |  | 2298 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 14, sub: 0, line: 189 } |  |  | 0.629 |
| walker |  | 2309 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 16, sub: 0, line: 216 } |  |  | 0.637 |
| walker |  | 2346 | 37 | Code::CodeKey { rung: Names, file: nanovllm/layers/rotary_embedding.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 2370 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 2, sub: 0, line: 17 } |  |  | 0.644 |
| ns | 2388 |  | 212 | The Context dataclass and the utils/ function roster | 2.10 |  | 0.614 |
| walker |  | 2409 | 39 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.614 |
| walker |  | 2465 | 56 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 3, sub: 0, line: 19 } |  |  | 0.614 |
| walker |  | 2526 | 61 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.614 |
| ns | 2583 |  | 195 | Import blocks of llm_engine.py and scheduler.py | 2.11 |  | 0.588 |
| walker |  | 2596 | 70 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.591 |
| walker |  | 2616 | 20 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 3, sub: 0, line: 16 } |  |  | 0.591 |
| walker |  | 2672 | 56 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 2713 | 41 | Code::CodeKey { rung: Names, file: nanovllm/utils/loader.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 2723 | 10 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 1, sub: 0, line: 8 } |  |  | 0.591 |
| walker |  | 2776 | 53 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 4, sub: 0, line: 61 } |  |  | 0.591 |
| ns | 2778 |  | 195 | Import blocks of model_runner.py and block_manager.py | 2.12 |  | 0.568 |
| walker |  | 2868 | 92 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.570 |
| walker |  | 2897 | 29 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.570 |
| ns | 3012 |  | 234 | Import blocks of qwen3.py and attention.py | 2.13 |  | 0.548 |
| walker |  | 3076 | 179 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 5, sub: 0, line: 26 } |  |  | 0.584 |
| walker |  | 3084 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.589 |
| walker |  | 3099 | 15 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 10, sub: 0, line: 56 } |  |  | 0.589 |
| walker |  | 3121 | 22 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.589 |
| walker |  | 3186 | 65 | Code::CodeKey { rung: Names, file: nanovllm/layers/attention.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| ns | 3203 |  | 191 | pyproject project metadata and dependency list | 2.14 |  | 0.616 |
| walker |  | 3226 | 40 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 3, sub: 0, line: 43 } |  |  | 0.616 |
| walker |  | 3272 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.616 |
| ns | 3324 |  | 121 | README installation and model-download commands | 2.15 |  | 0.625 |
| walker |  | 3357 | 85 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| walker |  | 3393 | 36 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 6, sub: 0, line: 60 } |  |  | 0.627 |
| walker |  | 3450 | 57 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 7, sub: 0, line: 85 } |  |  | 0.627 |
| ns | 3458 |  | 134 | LLMEngine.step and is_finished | 3.1 | 2.3 | 0.617 |
| ns | 3604 |  | 146 | Scheduler construction and queue predicates | 3.2 | 2.4 | 0.606 |
| walker |  | 3676 | 226 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 2, sub: 0, line: 14 } |  |  | 0.646 |
| walker |  | 3684 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.649 |
| walker |  | 3692 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.653 |
| walker |  | 3700 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.656 |
| walker |  | 3708 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.660 |
| walker |  | 3716 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.663 |
| walker |  | 3724 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.667 |
| walker |  | 3732 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.671 |
| walker |  | 3771 | 39 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 3, sub: 0, line: 36 } |  |  | 0.672 |
| ns | 3841 |  | 237 | Scheduler.schedule — the prefill pass | 3.3 | 2.4 | 0.650 |
| walker |  | 3848 | 77 | Code::CodeKey { rung: Names, file: nanovllm/models/qwen3.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 3872 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 1, sub: 0, line: 14 } |  |  | 0.653 |
| walker |  | 3896 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 7, sub: 0, line: 119 } |  |  | 0.654 |
| walker |  | 3920 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 10, sub: 0, line: 161 } |  |  | 0.654 |
| walker |  | 3947 | 27 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 4, sub: 0, line: 90 } |  |  | 0.654 |
| walker |  | 3975 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 8, sub: 0, line: 121 } |  |  | 0.654 |
| walker |  | 4003 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 11, sub: 0, line: 163 } |  |  | 0.654 |
| walker |  | 4041 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 3, sub: 0, line: 71 } |  |  | 0.654 |
| ns | 4064 |  | 223 | Scheduler.schedule — the decode pass and preempt | 3.4 | 2.4 | 0.633 |
| walker |  | 4079 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 12, sub: 0, line: 172 } |  |  | 0.633 |
| walker |  | 4125 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 5, sub: 0, line: 92 } |  |  | 0.633 |
| ns | 4176 |  | 112 | Scheduler.postprocess — the stop condition | 3.5 | 2.4 | 0.627 |
| walker |  | 4180 | 55 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 9, sub: 0, line: 145 } |  |  | 0.627 |
| walker |  | 4316 | 136 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 13, sub: 0, line: 185 } |  |  | 0.648 |
| ns | 4335 |  | 159 | Sequence.__init__ — per-request state | 3.6 | 2.6 | 0.637 |
| walker |  | 4344 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 14, sub: 0, line: 194 } |  |  | 0.637 |
| walker |  | 4372 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.637 |
| walker |  | 4410 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.637 |
| ns | 4518 |  | 183 | LLMEngine.generate signature and request submission | 3.7 | 2.3 | 0.627 |
| walker |  | 4548 | 138 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 2, sub: 0, line: 16 } |  |  | 0.627 |
| walker |  | 4571 | 23 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 13, sub: 0, line: 93 } |  |  | 0.627 |
| walker |  | 4579 | 8 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 4, sub: 0, line: 31 } |  |  | 0.627 |
| walker |  | 4679 | 100 | Code::CodeKey { rung: Names, file: nanovllm/layers/linear.py, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 4713 | 34 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 2, sub: 0, line: 12 } |  |  | 0.651 |
| walker |  | 4757 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 13, sub: 0, line: 76 } |  |  | 0.651 |
| walker |  | 4801 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 16, sub: 0, line: 96 } |  |  | 0.651 |
| ns | 4810 |  | 292 | LLMEngine.generate — drive loop and output assembly | 3.8 | 3.7 | 0.632 |
| walker |  | 4846 | 45 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.632 |
| walker |  | 4903 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 5, sub: 0, line: 37 } |  |  | 0.632 |
| walker |  | 4947 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.632 |
| walker |  | 5004 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 9, sub: 0, line: 54 } |  |  | 0.632 |
| walker |  | 5048 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 10, sub: 0, line: 56 } |  |  | 0.632 |
| ns | 5059 |  | 249 | LLMEngine.__init__ — config filtering and TP worker spawn | 3.9 | 2.3 | 0.617 |
| walker |  | 5105 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 19, sub: 0, line: 131 } |  |  | 0.617 |
| walker |  | 5149 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 20, sub: 0, line: 133 } |  |  | 0.617 |
| ns | 5163 |  | 104 | LLMEngine.exit and add_request | 3.10 | 2.3 | 0.614 |
| walker |  | 5207 | 58 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 3, sub: 0, line: 14 } |  |  | 0.614 |
| walker |  | 5279 | 72 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 17, sub: 0, line: 98 } |  |  | 0.614 |
| ns | 5345 |  | 182 | BlockManager.__init__ and compute_hash | 3.11 | 2.5 | 0.604 |
| walker |  | 5381 | 102 | Code::CodeKey { rung: Names, file: nanovllm/utils/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5389 | 8 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 3, sub: 0, line: 18 } |  |  | 0.607 |
| walker |  | 5408 | 19 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 5, sub: 0, line: 25 } |  |  | 0.607 |
| walker |  | 5541 | 133 | Code::CodeKey { rung: Decl, file: nanovllm/utils/context.py, decl: 1, sub: 0, line: 5 } |  |  | 0.631 |
| walker |  | 5647 | 106 | Code::CodeKey { rung: Body, file: nanovllm/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.646 |
| ns | 5686 |  | 341 | BlockManager.can_allocate and allocate — the prefix-cache hit path | 3.12 | 2.5 | 0.625 |
| walker |  | 5703 | 56 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 4, sub: 0, line: 21 } |  |  | 0.625 |
| walker |  | 5766 | 63 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 13, sub: 0, line: 182 } |  |  | 0.625 |
| walker |  | 6045 | 279 | Code::CodeKey { rung: Body, file: example.py, decl: 1, sub: 0, line: 6 } |  |  | 0.625 |
| ns | 6057 |  | 371 | BlockManager.deallocate, can_append and may_append | 3.13 | 2.5 | 0.606 |
| ns | 6182 |  | 125 | Block class body | 3.14 | 2.5 | 0.599 |
| walker |  | 6198 | 153 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.623 |
| walker |  | 6243 | 45 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 4, sub: 0, line: 42 } |  |  | 0.632 |
| walker |  | 6276 | 33 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.632 |
| walker |  | 6285 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 5, sub: 0, line: 34 } |  |  | 0.632 |
| ns | 6344 |  | 162 | Sequence block arithmetic and append_token | 3.15 | 2.6 | 0.621 |
| walker |  | 6358 | 73 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.621 |
| walker |  | 6437 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.621 |
| ns | 6469 |  | 125 | Sequence.__getstate__ / __setstate__ — the TP pickling contract | 3.16 | 2.6 | 0.615 |
| walker |  | 6803 | 366 | Code::CodeKey { rung: Body, file: bench.py, decl: 1, sub: 0, line: 8 } |  |  | 0.615 |
| walker |  | 6843 | 40 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.615 |
| walker |  | 6874 | 31 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 4, sub: 0, line: 20 } |  |  | 0.620 |
| ns | 6881 |  | 412 | ModelRunner.run and run_model — the per-step dispatch | 4.1 | 2.7 | 0.605 |
| walker |  | 6882 | 8 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 4, sub: 0, line: 33 } |  |  | 0.605 |
| walker |  | 6969 | 87 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 10, sub: 0, line: 120 } |  |  | 0.605 |
| walker |  | 7191 | 222 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 7203 | 12 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.605 |
| walker |  | 7245 | 42 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.605 |
| walker |  | 7255 | 10 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 7, sub: 0, line: 47 } |  |  | 0.605 |
| ns | 7295 |  | 414 | ModelRunner.__init__ — distributed init and startup order | 4.2 | 2.7 | 0.587 |
| walker |  | 7348 | 93 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 7, sub: 0, line: 65 } |  |  | 0.596 |
| walker |  | 7439 | 91 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 5, sub: 0, line: 68 } |  |  | 0.596 |
| walker |  | 7452 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.596 |
| walker |  | 7523 | 71 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 3, sub: 0, line: 27 } |  |  | 0.596 |
| walker |  | 7566 | 43 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 2, sub: 0, line: 10 } |  |  | 0.604 |
| walker |  | 7579 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.604 |
| ns | 7645 |  | 350 | ModelRunner.allocate_kv_cache — sizing the paged cache | 4.3 | 2.7 | 0.594 |
| walker |  | 7678 | 99 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.594 |
| walker |  | 7689 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.594 |
| walker |  | 7702 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.594 |
| walker |  | 7805 | 103 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.605 |
| walker |  | 7819 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 8, sub: 0, line: 50 } |  |  | 0.605 |
| walker |  | 8046 | 227 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 2, sub: 0, line: 12 } |  |  | 0.605 |
| ns | 8059 |  | 414 | ModelRunner.prepare_prefill — varlen batching and slot mapping | 4.4 | 2.7 | 0.592 |
| walker |  | 8125 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.592 |
| ns | 8235 |  | 176 | ModelRunner.prepare_decode — one token per sequence | 4.5 | 2.7 | 0.585 |
| walker |  | 8285 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.585 |
| walker |  | 8296 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.585 |
| walker |  | 8309 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.586 |
| walker |  | 8427 | 118 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 5, sub: 0, line: 48 } |  |  | 0.596 |
| walker |  | 8441 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 12, sub: 0, line: 72 } |  |  | 0.596 |
| walker |  | 8545 | 104 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 6, sub: 0, line: 76 } |  |  | 0.596 |
| ns | 8552 |  | 317 | ModelRunner.capture_cudagraph — the graph ladder | 4.6 | 2.7 | 0.585 |
| walker |  | 8625 | 80 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.585 |
| ns | 8701 |  | 149 | Complete method roster for layers/linear.py | 5.1 | 2.8 | 0.593 |
| walker |  | 8707 | 82 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.593 |
| walker |  | 8771 | 64 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.597 |
| walker |  | 8786 | 15 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.597 |
| ns | 8810 |  | 109 | Complete method roster for models/qwen3.py | 5.2 | 2.9 | 0.602 |
| walker |  | 8901 | 115 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 4, sub: 0, line: 34 } |  |  | 0.602 |
| walker |  | 9062 | 161 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 2, sub: 0, line: 33 } |  |  | 0.603 |
| ns | 9141 |  | 331 | Attention.forward — paged KV write and flash-attn dispatch | 5.3 | 2.8 | 0.595 |
| walker |  | 9173 | 111 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 15, sub: 0, line: 208 } |  |  | 0.598 |
| walker |  | 9194 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.600 |
| walker |  | 9318 | 124 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 8, sub: 0, line: 91 } |  |  | 0.600 |
| ns | 9387 |  | 246 | Qwen3Attention.forward — QKV split, q/k norm, RoPE | 5.4 | 5.2 | 0.593 |
| walker |  | 9420 | 102 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.593 |
| walker |  | 9441 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.595 |
| walker |  | 9462 | 21 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 1, sub: 0, line: 7 } |  |  | 0.595 |
| walker |  | 9548 | 86 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 12, sub: 0, line: 84 } |  |  | 0.598 |
| ns | 9575 |  | 188 | store_kvcache and its Triton kernel boundary | 5.5 | 2.8 | 0.602 |
| ns | 9684 |  | 109 | Sampler.forward — the exponential-noise argmax trick | 5.6 | 2.8 | 0.605 |
| walker |  | 9708 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.605 |
| walker |  | 9796 | 88 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 6, sub: 0, line: 28 } |  |  | 0.614 |
| ns | 9865 |  | 181 | ParallelLMHead.forward — last-token selection and logit gather | 5.7 | 2.8 | 0.609 |
| ns | 9974 |  | 109 | README benchmark pointer and throughput table | 6.1 |  | 0.611 |

Score(3000)=0.708 I=0.864 C=0.580 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.736/0.746/0.749/0.708/0.671/0.603/0.600

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
| walker |  | 413 | 22 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.893 |
| ns | 450 |  | 148 | README Quick Start usage snippet | 1.7 |  | 0.779 |
| walker |  | 488 | 75 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.782 |
| walker |  | 527 | 39 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.782 |
| walker |  | 538 | 11 | Code::CodeKey { rung: Names, file: bench.py, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| walker |  | 549 | 11 | Code::CodeKey { rung: Names, file: example.py, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| ns | 567 |  | 117 | SamplingParams in full | 1.8 |  | 0.684 |
| walker |  | 580 | 31 | Code::CodeKey { rung: Names, file: nanovllm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 636 | 56 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 647 | 11 | Code::CodeKey { rung: Names, file: nanovllm/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| ns | 781 |  | 214 | Config dataclass — every field with its default | 1.9 |  | 0.591 |
| walker |  | 831 | 184 | Code::CodeKey { rung: Decl, file: nanovllm/config.py, decl: 1, sub: 0, line: 6 } |  |  | 0.700 |
| ns | 899 |  | 118 | Config.__post_init__ validation and derivation | 1.10 |  | 0.664 |
| walker |  | 923 | 92 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.681 |
| ns | 923 |  | 24 | Complete nanovllm/engine/ listing | 2.1 |  | 0.681 |
| walker |  | 935 | 12 | Code::CodeKey { rung: Names, file: nanovllm/sampling_params.py, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| ns | 972 |  | 49 | Listings for layers/, models/, utils/ and assets/ | 2.2 |  | 0.706 |
| walker |  | 1000 | 65 | Code::CodeKey { rung: Decl, file: nanovllm/sampling_params.py, decl: 1, sub: 0, line: 4 } |  |  | 0.736 |
| walker |  | 1021 | 21 | Code::CodeKey { rung: Body, file: nanovllm/sampling_params.py, decl: 2, sub: 0, line: 10 } |  |  | 0.758 |
| walker |  | 1037 | 16 | Code::CodeKey { rung: Names, file: nanovllm/llm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| walker |  | 1042 | 5 | Code::CodeKey { rung: Decl, file: nanovllm/llm.py, decl: 1, sub: 0, line: 4 } |  |  | 0.774 |
| ns | 1047 |  | 75 | LLMEngine method roster | 2.3 |  | 0.742 |
| walker |  | 1063 | 21 | Code::CodeKey { rung: Names, file: nanovllm/engine/block_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 1114 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 1, sub: 0, line: 8 } |  |  | 0.743 |
| ns | 1121 |  | 74 | Scheduler method roster | 2.4 |  | 0.715 |
| walker |  | 1134 | 20 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 3, sub: 0, line: 16 } |  |  | 0.715 |
| ns | 1280 |  | 159 | Block and BlockManager method roster | 2.5 |  | 0.671 |
| walker |  | 1313 | 179 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 5, sub: 0, line: 26 } |  |  | 0.735 |
| walker |  | 1321 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.745 |
| walker |  | 1336 | 15 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 10, sub: 0, line: 56 } |  |  | 0.745 |
| walker |  | 1359 | 23 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 13, sub: 0, line: 93 } |  |  | 0.745 |
| walker |  | 1381 | 22 | Code::CodeKey { rung: Names, file: nanovllm/engine/sequence.py, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1409 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 1, sub: 0, line: 8 } |  |  | 0.746 |
| ns | 1569 |  | 289 | SequenceStatus members and the complete Sequence member roster | 2.6 |  | 0.666 |
| walker |  | 1635 | 226 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 2, sub: 0, line: 14 } |  |  | 0.740 |
| walker |  | 1643 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.746 |
| walker |  | 1651 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.752 |
| walker |  | 1659 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.758 |
| walker |  | 1667 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.765 |
| walker |  | 1675 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.771 |
| walker |  | 1683 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.778 |
| walker |  | 1691 | 8 | Code::CodeKey { rung: Decl, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.785 |
| walker |  | 1699 | 8 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 4, sub: 0, line: 31 } |  |  | 0.785 |
| walker |  | 1710 | 11 | Code::CodeKey { rung: Names, file: nanovllm/engine/scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| ns | 1772 |  | 203 | ModelRunner method roster | 2.7 |  | 0.736 |
| walker |  | 1822 | 112 | Code::CodeKey { rung: Decl, file: nanovllm/engine/scheduler.py, decl: 1, sub: 0, line: 8 } |  |  | 0.764 |
| walker |  | 1831 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 4, sub: 0, line: 21 } |  |  | 0.765 |
| walker |  | 1844 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 3, sub: 0, line: 18 } |  |  | 0.765 |
| walker |  | 1880 | 36 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 6, sub: 0, line: 60 } |  |  | 0.765 |
| walker |  | 1986 | 106 | Code::CodeKey { rung: Body, file: nanovllm/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.795 |
| walker |  | 1998 | 12 | Code::CodeKey { rung: Names, file: nanovllm/engine/model_runner.py, decl: 0, sub: 0, line: 0 } |  |  | 0.795 |
| ns | 2006 |  | 234 | Complete top-level symbol roster for nanovllm/layers/ | 2.8 |  | 0.745 |
| ns | 2176 |  | 170 | qwen3.py class roster and packed_modules_mapping | 2.9 |  | 0.719 |
| walker |  | 2277 | 279 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 1, sub: 0, line: 15 } |  |  | 0.764 |
| walker |  | 2288 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 14, sub: 0, line: 189 } |  |  | 0.771 |
| walker |  | 2299 | 11 | Code::CodeKey { rung: Decl, file: nanovllm/engine/model_runner.py, decl: 16, sub: 0, line: 216 } |  |  | 0.778 |
| walker |  | 2352 | 53 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 4, sub: 0, line: 61 } |  |  | 0.778 |
| ns | 2388 |  | 212 | The Context dataclass and the utils/ function roster | 2.10 |  | 0.743 |
| walker |  | 2409 | 57 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 7, sub: 0, line: 85 } |  |  | 0.743 |
| walker |  | 2472 | 63 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 13, sub: 0, line: 182 } |  |  | 0.743 |
| walker |  | 2509 | 37 | Code::CodeKey { rung: Names, file: nanovllm/layers/rotary_embedding.py, decl: 0, sub: 0, line: 0 } |  |  | 0.744 |
| walker |  | 2533 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 2, sub: 0, line: 17 } |  |  | 0.744 |
| walker |  | 2572 | 39 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.744 |
| ns | 2583 |  | 195 | Import blocks of llm_engine.py and scheduler.py | 2.11 |  | 0.713 |
| walker |  | 2628 | 56 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 3, sub: 0, line: 19 } |  |  | 0.713 |
| walker |  | 2689 | 61 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.713 |
| walker |  | 2759 | 70 | Code::CodeKey { rung: Decl, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.714 |
| walker |  | 2772 | 13 | Code::CodeKey { rung: Names, file: nanovllm/engine/llm_engine.py, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| ns | 2778 |  | 195 | Import blocks of model_runner.py and block_manager.py | 2.12 |  | 0.686 |
| walker |  | 2862 | 90 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 1, sub: 0, line: 15 } |  |  | 0.707 |
| walker |  | 2871 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 6, sub: 0, line: 56 } |  |  | 0.707 |
| walker |  | 2933 | 62 | Code::CodeKey { rung: Decl, file: nanovllm/engine/llm_engine.py, decl: 7, sub: 0, line: 59 } |  |  | 0.708 |
| walker |  | 2972 | 39 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 3, sub: 0, line: 36 } |  |  | 0.708 |
| ns | 3012 |  | 234 | Import blocks of qwen3.py and attention.py | 2.13 |  | 0.680 |
| ns | 3203 |  | 191 | pyproject project metadata and dependency list | 2.14 |  | 0.693 |
| walker |  | 3251 | 279 | Code::CodeKey { rung: Body, file: example.py, decl: 1, sub: 0, line: 6 } |  |  | 0.693 |
| ns | 3324 |  | 121 | README installation and model-download commands | 2.15 |  | 0.699 |
| walker |  | 3404 | 153 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.732 |
| walker |  | 3418 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/layernorm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| ns | 3458 |  | 134 | LLMEngine.step and is_finished | 3.1 | 2.3 | 0.720 |
| walker |  | 3466 | 48 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 1, sub: 0, line: 5 } |  |  | 0.720 |
| walker |  | 3502 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.720 |
| walker |  | 3543 | 41 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.720 |
| walker |  | 3592 | 49 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.720 |
| ns | 3604 |  | 146 | Scheduler construction and queue predicates | 3.2 | 2.4 | 0.706 |
| walker |  | 3643 | 51 | Code::CodeKey { rung: Decl, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.706 |
| walker |  | 3657 | 14 | Code::CodeKey { rung: Names, file: nanovllm/layers/sampler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 3694 | 37 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 1, sub: 0, line: 5 } |  |  | 0.707 |
| walker |  | 3703 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.707 |
| walker |  | 3710 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 2, sub: 0, line: 7 } |  |  | 0.707 |
| walker |  | 3810 | 100 | Code::CodeKey { rung: Names, file: nanovllm/layers/linear.py, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| ns | 3841 |  | 237 | Scheduler.schedule — the prefill pass | 3.3 | 2.4 | 0.700 |
| walker |  | 3844 | 34 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 2, sub: 0, line: 12 } |  |  | 0.700 |
| walker |  | 3888 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 13, sub: 0, line: 76 } |  |  | 0.700 |
| walker |  | 3932 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 16, sub: 0, line: 96 } |  |  | 0.700 |
| walker |  | 3977 | 45 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.700 |
| walker |  | 4034 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 5, sub: 0, line: 37 } |  |  | 0.700 |
| ns | 4064 |  | 223 | Scheduler.schedule — the decode pass and preempt | 3.4 | 2.4 | 0.678 |
| walker |  | 4078 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.678 |
| walker |  | 4135 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 9, sub: 0, line: 54 } |  |  | 0.678 |
| ns | 4176 |  | 112 | Scheduler.postprocess — the stop condition | 3.5 | 2.4 | 0.671 |
| walker |  | 4179 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 10, sub: 0, line: 56 } |  |  | 0.671 |
| walker |  | 4236 | 57 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 19, sub: 0, line: 131 } |  |  | 0.671 |
| walker |  | 4280 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 20, sub: 0, line: 133 } |  |  | 0.671 |
| ns | 4335 |  | 159 | Sequence.__init__ — per-request state | 3.6 | 2.6 | 0.659 |
| walker |  | 4338 | 58 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 3, sub: 0, line: 14 } |  |  | 0.659 |
| walker |  | 4410 | 72 | Code::CodeKey { rung: Decl, file: nanovllm/layers/linear.py, decl: 17, sub: 0, line: 98 } |  |  | 0.659 |
| walker |  | 4455 | 45 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 4, sub: 0, line: 42 } |  |  | 0.660 |
| walker |  | 4488 | 33 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 2, sub: 0, line: 7 } |  |  | 0.660 |
| walker |  | 4497 | 9 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 5, sub: 0, line: 34 } |  |  | 0.660 |
| ns | 4518 |  | 183 | LLMEngine.generate signature and request submission | 3.7 | 2.3 | 0.650 |
| walker |  | 4574 | 77 | Code::CodeKey { rung: Names, file: nanovllm/models/qwen3.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4598 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 1, sub: 0, line: 14 } |  |  | 0.653 |
| walker |  | 4622 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 7, sub: 0, line: 119 } |  |  | 0.653 |
| walker |  | 4646 | 24 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 10, sub: 0, line: 161 } |  |  | 0.653 |
| walker |  | 4673 | 27 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 4, sub: 0, line: 90 } |  |  | 0.653 |
| walker |  | 4701 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 8, sub: 0, line: 121 } |  |  | 0.653 |
| walker |  | 4729 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 11, sub: 0, line: 163 } |  |  | 0.653 |
| walker |  | 4767 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 3, sub: 0, line: 71 } |  |  | 0.654 |
| walker |  | 4805 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 12, sub: 0, line: 172 } |  |  | 0.654 |
| ns | 4810 |  | 292 | LLMEngine.generate — drive loop and output assembly | 3.8 | 3.7 | 0.634 |
| walker |  | 4851 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 5, sub: 0, line: 92 } |  |  | 0.634 |
| walker |  | 4906 | 55 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 9, sub: 0, line: 145 } |  |  | 0.634 |
| walker |  | 5042 | 136 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 13, sub: 0, line: 185 } |  |  | 0.654 |
| ns | 5059 |  | 249 | LLMEngine.__init__ — config filtering and TP worker spawn | 3.9 | 2.3 | 0.639 |
| walker |  | 5070 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 14, sub: 0, line: 194 } |  |  | 0.639 |
| walker |  | 5098 | 28 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.639 |
| walker |  | 5136 | 38 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.639 |
| ns | 5163 |  | 104 | LLMEngine.exit and add_request | 3.10 | 2.3 | 0.644 |
| walker |  | 5274 | 138 | Code::CodeKey { rung: Decl, file: nanovllm/models/qwen3.py, decl: 2, sub: 0, line: 16 } |  |  | 0.644 |
| walker |  | 5305 | 31 | Code::CodeKey { rung: Names, file: nanovllm/layers/embed_head.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 5335 | 30 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 5, sub: 0, line: 45 } |  |  | 0.650 |
| ns | 5345 |  | 182 | BlockManager.__init__ and compute_hash | 3.11 | 2.5 | 0.640 |
| walker |  | 5379 | 44 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.640 |
| walker |  | 5432 | 53 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 1, sub: 0, line: 9 } |  |  | 0.640 |
| walker |  | 5465 | 33 | Code::CodeKey { rung: Decl, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.640 |
| walker |  | 5487 | 22 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 6, sub: 0, line: 47 } |  |  | 0.640 |
| walker |  | 5503 | 16 | Code::CodeKey { rung: Names, file: nanovllm/layers/activation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 5539 | 36 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 1, sub: 0, line: 6 } |  |  | 0.642 |
| walker |  | 5548 | 9 | Code::CodeKey { rung: Decl, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.642 |
| walker |  | 5555 | 7 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 2, sub: 0, line: 8 } |  |  | 0.642 |
| walker |  | 5584 | 29 | Code::CodeKey { rung: Body, file: nanovllm/layers/activation.py, decl: 3, sub: 0, line: 11 } |  |  | 0.642 |
| walker |  | 5663 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/sampler.py, decl: 3, sub: 0, line: 10 } |  |  | 0.643 |
| ns | 5686 |  | 341 | BlockManager.can_allocate and allocate — the prefix-cache hit path | 3.12 | 2.5 | 0.622 |
| walker |  | 6029 | 366 | Code::CodeKey { rung: Body, file: bench.py, decl: 1, sub: 0, line: 8 } |  |  | 0.622 |
| ns | 6057 |  | 371 | BlockManager.deallocate, can_append and may_append | 3.13 | 2.5 | 0.603 |
| walker |  | 6069 | 40 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 5, sub: 0, line: 51 } |  |  | 0.603 |
| walker |  | 6100 | 31 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 4, sub: 0, line: 20 } |  |  | 0.603 |
| walker |  | 6108 | 8 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 4, sub: 0, line: 33 } |  |  | 0.603 |
| ns | 6182 |  | 125 | Block class body | 3.14 | 2.5 | 0.602 |
| walker |  | 6210 | 102 | Code::CodeKey { rung: Names, file: nanovllm/utils/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 6218 | 8 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 3, sub: 0, line: 18 } |  |  | 0.603 |
| walker |  | 6237 | 19 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 5, sub: 0, line: 25 } |  |  | 0.603 |
| ns | 6344 |  | 162 | Sequence block arithmetic and append_token | 3.15 | 2.6 | 0.593 |
| walker |  | 6370 | 133 | Code::CodeKey { rung: Decl, file: nanovllm/utils/context.py, decl: 1, sub: 0, line: 5 } |  |  | 0.610 |
| walker |  | 6426 | 56 | Code::CodeKey { rung: Body, file: nanovllm/utils/context.py, decl: 4, sub: 0, line: 21 } |  |  | 0.610 |
| walker |  | 6467 | 41 | Code::CodeKey { rung: Names, file: nanovllm/utils/loader.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| ns | 6469 |  | 125 | Sequence.__getstate__ / __setstate__ — the TP pickling contract | 3.16 | 2.6 | 0.609 |
| walker |  | 6477 | 10 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 1, sub: 0, line: 8 } |  |  | 0.609 |
| walker |  | 6542 | 65 | Code::CodeKey { rung: Names, file: nanovllm/layers/attention.py, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 6582 | 40 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 3, sub: 0, line: 43 } |  |  | 0.617 |
| walker |  | 6628 | 46 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.617 |
| walker |  | 6713 | 85 | Code::CodeKey { rung: Decl, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.620 |
| walker |  | 6786 | 73 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 4, sub: 0, line: 45 } |  |  | 0.620 |
| walker |  | 6873 | 87 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 10, sub: 0, line: 120 } |  |  | 0.620 |
| ns | 6881 |  | 412 | ModelRunner.run and run_model — the per-step dispatch | 4.1 | 2.7 | 0.605 |
| walker |  | 7095 | 222 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 7107 | 12 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 9, sub: 0, line: 49 } |  |  | 0.605 |
| walker |  | 7149 | 42 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 5, sub: 0, line: 42 } |  |  | 0.605 |
| walker |  | 7159 | 10 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 7, sub: 0, line: 47 } |  |  | 0.605 |
| walker |  | 7252 | 93 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 7, sub: 0, line: 65 } |  |  | 0.614 |
| ns | 7295 |  | 414 | ModelRunner.__init__ — distributed init and startup order | 4.2 | 2.7 | 0.596 |
| walker |  | 7343 | 91 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 5, sub: 0, line: 68 } |  |  | 0.596 |
| walker |  | 7356 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 6, sub: 0, line: 37 } |  |  | 0.596 |
| walker |  | 7427 | 71 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 3, sub: 0, line: 27 } |  |  | 0.596 |
| walker |  | 7469 | 42 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 9, sub: 0, line: 51 } |  |  | 0.596 |
| walker |  | 7482 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 7, sub: 0, line: 41 } |  |  | 0.596 |
| walker |  | 7581 | 99 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.596 |
| walker |  | 7592 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 15, sub: 0, line: 204 } |  |  | 0.596 |
| walker |  | 7605 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 8, sub: 0, line: 45 } |  |  | 0.596 |
| ns | 7645 |  | 350 | ModelRunner.allocate_kv_cache — sizing the paged cache | 4.3 | 2.7 | 0.586 |
| walker |  | 7648 | 43 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 2, sub: 0, line: 10 } |  |  | 0.594 |
| walker |  | 7751 | 103 | Code::CodeKey { rung: Body, file: nanovllm/engine/scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.605 |
| walker |  | 7765 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 8, sub: 0, line: 50 } |  |  | 0.605 |
| walker |  | 7992 | 227 | Code::CodeKey { rung: Body, file: nanovllm/utils/loader.py, decl: 2, sub: 0, line: 12 } |  |  | 0.605 |
| ns | 8059 |  | 414 | ModelRunner.prepare_prefill — varlen batching and slot mapping | 4.4 | 2.7 | 0.592 |
| walker |  | 8071 | 79 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 4, sub: 0, line: 37 } |  |  | 0.592 |
| walker |  | 8231 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 1, sub: 0, line: 10 } |  |  | 0.592 |
| ns | 8235 |  | 176 | ModelRunner.prepare_decode — one token per sequence | 4.5 | 2.7 | 0.585 |
| walker |  | 8242 | 11 | Code::CodeKey { rung: Body, file: nanovllm/models/qwen3.py, decl: 16, sub: 0, line: 211 } |  |  | 0.585 |
| walker |  | 8255 | 13 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 10, sub: 0, line: 53 } |  |  | 0.586 |
| walker |  | 8373 | 118 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 5, sub: 0, line: 48 } |  |  | 0.596 |
| walker |  | 8387 | 14 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 12, sub: 0, line: 72 } |  |  | 0.596 |
| walker |  | 8491 | 104 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 6, sub: 0, line: 76 } |  |  | 0.596 |
| ns | 8552 |  | 317 | ModelRunner.capture_cudagraph — the graph ladder | 4.6 | 2.7 | 0.585 |
| walker |  | 8571 | 80 | Code::CodeKey { rung: Body, file: nanovllm/layers/rotary_embedding.py, decl: 1, sub: 0, line: 6 } |  |  | 0.585 |
| walker |  | 8653 | 82 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 3, sub: 0, line: 16 } |  |  | 0.585 |
| walker |  | 8668 | 15 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 6, sub: 0, line: 39 } |  |  | 0.585 |
| ns | 8701 |  | 149 | Complete method roster for layers/linear.py | 5.1 | 2.8 | 0.593 |
| walker |  | 8783 | 115 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 4, sub: 0, line: 34 } |  |  | 0.593 |
| ns | 8810 |  | 109 | Complete method roster for models/qwen3.py | 5.2 | 2.9 | 0.598 |
| walker |  | 8944 | 161 | Code::CodeKey { rung: Body, file: nanovllm/layers/attention.py, decl: 2, sub: 0, line: 33 } |  |  | 0.599 |
| walker |  | 9055 | 111 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 15, sub: 0, line: 208 } |  |  | 0.602 |
| walker |  | 9119 | 64 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 7, sub: 0, line: 35 } |  |  | 0.606 |
| walker |  | 9140 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 11, sub: 0, line: 57 } |  |  | 0.608 |
| ns | 9141 |  | 331 | Attention.forward — paged KV write and flash-attn dispatch | 5.3 | 2.8 | 0.600 |
| walker |  | 9264 | 124 | Code::CodeKey { rung: Body, file: nanovllm/engine/model_runner.py, decl: 8, sub: 0, line: 91 } |  |  | 0.600 |
| walker |  | 9366 | 102 | Code::CodeKey { rung: Body, file: nanovllm/layers/layernorm.py, decl: 4, sub: 0, line: 28 } |  |  | 0.600 |
| walker |  | 9387 | 21 | Code::CodeKey { rung: Body, file: nanovllm/engine/sequence.py, decl: 12, sub: 0, line: 61 } |  |  | 0.595 |
| ns | 9387 |  | 246 | Qwen3Attention.forward — QKV split, q/k norm, RoPE | 5.4 | 5.2 | 0.595 |
| walker |  | 9408 | 21 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 1, sub: 0, line: 7 } |  |  | 0.595 |
| walker |  | 9478 | 70 | Code::CodeKey { rung: Body, file: nanovllm/engine/block_manager.py, decl: 8, sub: 0, line: 43 } |  |  | 0.595 |
| ns | 9575 |  | 188 | store_kvcache and its Triton kernel boundary | 5.5 | 2.8 | 0.599 |
| walker |  | 9638 | 160 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 2, sub: 0, line: 11 } |  |  | 0.599 |
| ns | 9684 |  | 109 | Sampler.forward — the exponential-noise argmax trick | 5.6 | 2.8 | 0.601 |
| ns | 9865 |  | 181 | ParallelLMHead.forward — last-token selection and logit gather | 5.7 | 2.8 | 0.597 |
| walker |  | 9881 | 243 | Code::CodeKey { rung: Body, file: nanovllm/engine/llm_engine.py, decl: 2, sub: 0, line: 17 } |  |  | 0.614 |
| walker |  | 9909 | 28 | Code::CodeKey { rung: Body, file: nanovllm/layers/linear.py, decl: 14, sub: 0, line: 78 } |  |  | 0.614 |
| ns | 9974 |  | 109 | README benchmark pointer and throughput table | 6.1 |  | 0.616 |
| walker |  | 9993 | 84 | Code::CodeKey { rung: Body, file: nanovllm/layers/embed_head.py, decl: 7, sub: 0, line: 56 } |  |  | 0.620 |

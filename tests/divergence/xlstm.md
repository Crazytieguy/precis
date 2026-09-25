Score(3000)=0.491 I=0.532 C=0.454 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.668/0.603/0.491/0.401/0.381/0.375

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 74 | 9 | Fs::DirListing { dir: notebooks } |  |  | 0.000 |
| walker |  | 77 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 85 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| walker |  | 113 | 28 | Fs::DirListing { dir: res } |  |  | 0.000 |
| walker |  | 128 | 15 | Fs::DirListing { dir: notebooks/xlstm_large } |  |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.594 |
| walker |  | 164 | 36 | Fs::DirListing { dir: xlstm } |  |  | 0.608 |
| walker |  | 186 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.625 |
| walker |  | 209 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.633 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.582 |
| walker |  | 232 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.592 |
| walker |  | 254 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.592 |
| walker |  | 270 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.592 |
| walker |  | 287 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 315 | 28 | Fs::DirListing { dir: xlstm/xlstm_large } |  |  | 0.621 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.528 |
| walker |  | 350 | 35 | Fs::DirListing { dir: xlstm/components } |  |  | 0.676 |
| walker |  | 396 | 46 | Fs::DirListing { dir: experiments } |  |  | 0.679 |
| walker |  | 412 | 16 | Fs::DirListing { dir: experiments/data } |  |  | 0.682 |
| walker |  | 438 | 26 | Fs::DirListing { dir: experiments/data/formal_language } |  |  | 0.689 |
| walker |  | 466 | 28 | Fs::DirListing { dir: experiments/data/formal_language/tasks } |  |  | 0.698 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.629 |
| walker |  | 551 | 85 | Toml::Identity { file: pyproject.toml } |  |  | 0.629 |
| walker |  | 584 | 33 | Fs::DirListing { dir: notebooks/xlstm } |  |  | 0.635 |
| walker |  | 687 | 103 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 742 | 55 | Fs::DirListing { dir: tests } |  |  | 0.582 |
| walker |  | 928 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.699 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.641 |
| walker |  | 1115 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1171 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.662 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.668 |
| walker |  | 1283 | 112 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 1320 | 37 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 1, sub: 0, line: 11 } |  |  | 0.668 |
| walker |  | 1494 | 174 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.668 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.615 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.594 |
| walker |  | 1670 | 176 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 2, sub: 0, line: 17 } |  |  | 0.594 |
| walker |  | 1712 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.566 |
| walker |  | 1872 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1954 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.603 |
| walker |  | 2109 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.603 |
| walker |  | 2183 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.604 |
| walker |  | 2197 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.557 |
| walker |  | 2300 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.557 |
| walker |  | 2309 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.557 |
| walker |  | 2331 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.557 |
| walker |  | 2354 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.557 |
| walker |  | 2394 | 40 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 3, sub: 0, line: 17 } |  |  | 0.557 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.531 |
| walker |  | 2484 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.531 |
| walker |  | 2515 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 2638 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.531 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.511 |
| walker |  | 2765 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.511 |
| walker |  | 2804 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.511 |
| walker |  | 2814 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.511 |
| walker |  | 2868 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.511 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.491 |
| walker |  | 2924 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.491 |
| walker |  | 2955 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.478 |
| walker |  | 3065 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.478 |
| walker |  | 3121 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.478 |
| walker |  | 3167 | 46 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 8, sub: 0, line: 111 } |  |  | 0.478 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.459 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.447 |
| walker |  | 3474 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.450 |
| walker |  | 3480 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.450 |
| walker |  | 3501 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.450 |
| walker |  | 3516 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.450 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.433 |
| walker |  | 3713 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.438 |
| walker |  | 3778 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| walker |  | 3795 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.438 |
| walker |  | 3839 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.438 |
| walker |  | 3867 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.438 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.422 |
| walker |  | 3950 | 83 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 7, sub: 0, line: 77 } |  |  | 0.422 |
| walker |  | 4152 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.422 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.401 |
| walker |  | 4319 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.401 |
| walker |  | 4359 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.401 |
| walker |  | 4412 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.401 |
| walker |  | 4421 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.393 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.393 |
| walker |  | 4430 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.393 |
| walker |  | 4440 | 10 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.393 |
| walker |  | 4515 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.393 |
| walker |  | 4523 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.393 |
| walker |  | 4537 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.393 |
| walker |  | 4551 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.393 |
| walker |  | 4567 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.393 |
| walker |  | 4575 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.393 |
| walker |  | 4587 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.393 |
| walker |  | 4633 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 4691 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.394 |
| walker |  | 4811 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.394 |
| walker |  | 4870 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.394 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.381 |
| walker |  | 4928 | 58 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 5, sub: 0, line: 49 } |  |  | 0.381 |
| walker |  | 4998 | 70 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.381 |
| walker |  | 5052 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.381 |
| walker |  | 5100 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.381 |
| walker |  | 5106 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.381 |
| walker |  | 5114 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.381 |
| walker |  | 5122 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.381 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.370 |
| walker |  | 5189 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.370 |
| walker |  | 5197 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.370 |
| walker |  | 5205 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.370 |
| walker |  | 5282 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.370 |
| walker |  | 5297 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.370 |
| walker |  | 5314 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.370 |
| walker |  | 5331 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.370 |
| walker |  | 5348 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.370 |
| walker |  | 5353 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.370 |
| walker |  | 5395 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.392 |
| walker |  | 5501 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.392 |
| walker |  | 5509 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.392 |
| walker |  | 5517 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.392 |
| walker |  | 5525 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.392 |
| walker |  | 5587 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.392 |
| walker |  | 5611 | 24 | Code::CodeKey { rung: Doc, file: xlstm/components/util.py, decl: 2, sub: 0, line: 11 } |  |  | 0.392 |
| walker |  | 5627 | 16 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 11, sub: 0, line: 73 } |  |  | 0.392 |
| walker |  | 5641 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.392 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.387 |
| walker |  | 5756 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.387 |
| walker |  | 5799 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 5862 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.390 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.381 |
| walker |  | 5958 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.381 |
| walker |  | 5978 | 20 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 2, sub: 0, line: 20 } |  |  | 0.381 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.369 |
| walker |  | 6269 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.369 |
| walker |  | 6295 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.369 |
| walker |  | 6345 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.369 |
| walker |  | 6401 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.369 |
| walker |  | 6409 | 8 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 6, sub: 0, line: 95 } |  |  | 0.369 |
| walker |  | 6438 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.371 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.364 |
| walker |  | 6555 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.364 |
| walker |  | 6692 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.365 |
| walker |  | 6745 | 53 | Code::CodeKey { rung: Doc, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.365 |
| walker |  | 6804 | 59 | Code::CodeKey { rung: Body, file: xlstm/blocks/xlstm_block.py, decl: 7, sub: 0, line: 89 } |  |  | 0.365 |
| walker |  | 6833 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.368 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.364 |
| walker |  | 6920 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.364 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.365 |
| walker |  | 7177 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.365 |
| walker |  | 7230 | 53 | Code::CodeKey { rung: Doc, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.365 |
| walker |  | 7259 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.368 |
| walker |  | 7282 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.368 |
| walker |  | 7341 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.368 |
| walker |  | 7349 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.368 |
| walker |  | 7423 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.360 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.360 |
| walker |  | 7443 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.360 |
| walker |  | 7448 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.360 |
| walker |  | 7466 | 18 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 1, sub: 0, line: 7 } |  |  | 0.360 |
| walker |  | 7565 | 99 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.360 |
| walker |  | 7580 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.360 |
| walker |  | 7609 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.360 |
| walker |  | 7624 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.360 |
| walker |  | 7641 | 17 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.360 |
| walker |  | 7661 | 20 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.360 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.359 |
| walker |  | 7822 | 161 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.360 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.355 |
| walker |  | 7900 | 78 | Code::CodeKey { rung: Body, file: xlstm/components/linear_headwise.py, decl: 5, sub: 0, line: 67 } |  |  | 0.355 |
| walker |  | 8001 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| walker |  | 8022 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.365 |
| walker |  | 8043 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.365 |
| walker |  | 8086 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.365 |
| walker |  | 8130 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.365 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.358 |
| walker |  | 8177 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.358 |
| walker |  | 8224 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.358 |
| walker |  | 8278 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.358 |
| walker |  | 8287 | 9 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.358 |
| walker |  | 8365 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.358 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.353 |
| walker |  | 8453 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.353 |
| walker |  | 8541 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.353 |
| walker |  | 8549 | 8 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.353 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.348 |
| walker |  | 8719 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.358 |
| walker |  | 8749 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.358 |
| walker |  | 8779 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.358 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.376 |
| walker |  | 8819 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.376 |
| walker |  | 8861 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.377 |
| walker |  | 8909 | 48 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.377 |
| walker |  | 8959 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.377 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.375 |
| walker |  | 9001 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.375 |
| walker |  | 9061 | 60 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.376 |
| walker |  | 9106 | 45 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.377 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.373 |
| walker |  | 9184 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.377 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.370 |
| walker |  | 9564 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.371 |
| walker |  | 9569 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.371 |
| walker |  | 9644 | 75 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 4, sub: 0, line: 41 } |  |  | 0.371 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.367 |
| walker |  | 9733 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.373 |
| walker |  | 9802 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.373 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.371 |
| walker |  | 9893 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.374 |
| walker |  | 9995 | 102 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.382 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.380 |

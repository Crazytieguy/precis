Score(3000)=0.496 I=0.542 C=0.454 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.668/0.567/0.496/0.402/0.405/0.395

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
| walker |  | 1743 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.566 |
| walker |  | 1853 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.566 |
| walker |  | 1909 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.566 |
| walker |  | 1955 | 46 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 8, sub: 0, line: 111 } |  |  | 0.566 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.522 |
| walker |  | 2262 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.526 |
| walker |  | 2268 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.526 |
| walker |  | 2289 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.526 |
| walker |  | 2304 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.526 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.502 |
| walker |  | 2464 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 2546 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.535 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.515 |
| walker |  | 2701 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.515 |
| walker |  | 2775 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.515 |
| walker |  | 2821 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 2879 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.515 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.496 |
| walker |  | 2999 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.496 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3058 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.483 |
| walker |  | 3116 | 58 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 5, sub: 0, line: 49 } |  |  | 0.483 |
| walker |  | 3186 | 70 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.483 |
| walker |  | 3200 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.464 |
| walker |  | 3303 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.464 |
| walker |  | 3312 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.464 |
| walker |  | 3334 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.464 |
| walker |  | 3357 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.464 |
| walker |  | 3397 | 40 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 3, sub: 0, line: 17 } |  |  | 0.464 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.451 |
| walker |  | 3487 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.451 |
| walker |  | 3518 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.451 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.434 |
| walker |  | 3641 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.434 |
| walker |  | 3768 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.434 |
| walker |  | 3807 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.434 |
| walker |  | 3817 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.434 |
| walker |  | 3871 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.434 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.419 |
| walker |  | 3927 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.419 |
| walker |  | 4124 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.423 |
| walker |  | 4189 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.423 |
| walker |  | 4206 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.423 |
| walker |  | 4250 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.423 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.402 |
| walker |  | 4278 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.402 |
| walker |  | 4361 | 83 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 7, sub: 0, line: 77 } |  |  | 0.402 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.394 |
| walker |  | 4563 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.394 |
| walker |  | 4730 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.394 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.381 |
| walker |  | 4900 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.396 |
| walker |  | 4930 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.396 |
| walker |  | 4960 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.396 |
| walker |  | 5000 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.396 |
| walker |  | 5042 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.398 |
| walker |  | 5090 | 48 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.398 |
| walker |  | 5140 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.399 |
| walker |  | 5182 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.400 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.388 |
| walker |  | 5242 | 60 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.389 |
| walker |  | 5287 | 45 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.390 |
| walker |  | 5365 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.397 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.415 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.410 |
| walker |  | 5745 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.412 |
| walker |  | 5785 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.412 |
| walker |  | 5838 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.412 |
| walker |  | 5847 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.412 |
| walker |  | 5856 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.412 |
| walker |  | 5866 | 10 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.412 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.402 |
| walker |  | 5941 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.402 |
| walker |  | 5949 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.402 |
| walker |  | 5963 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.402 |
| walker |  | 5977 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.402 |
| walker |  | 5993 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.402 |
| walker |  | 6001 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.402 |
| walker |  | 6013 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.402 |
| walker |  | 6088 | 75 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 4, sub: 0, line: 41 } |  |  | 0.403 |
| walker |  | 6177 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.405 |
| walker |  | 6246 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.405 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.393 |
| walker |  | 6337 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.393 |
| walker |  | 6470 | 133 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.394 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.387 |
| walker |  | 6522 | 52 | Code::CodeKey { rung: Body, file: xlstm/components/feedforward.py, decl: 9, sub: 0, line: 90 } |  |  | 0.387 |
| walker |  | 6575 | 53 | Code::CodeKey { rung: Body, file: xlstm/components/feedforward.py, decl: 4, sub: 0, line: 42 } |  |  | 0.387 |
| walker |  | 6628 | 53 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 9, sub: 0, line: 117 } |  |  | 0.393 |
| walker |  | 6682 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.393 |
| walker |  | 6730 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.393 |
| walker |  | 6736 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.393 |
| walker |  | 6744 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.393 |
| walker |  | 6752 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.393 |
| walker |  | 6819 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.393 |
| walker |  | 6827 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.393 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.387 |
| walker |  | 6835 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.387 |
| walker |  | 6912 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.387 |
| walker |  | 6927 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.387 |
| walker |  | 6944 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.387 |
| walker |  | 6961 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.387 |
| walker |  | 6978 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.387 |
| walker |  | 6983 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.387 |
| walker |  | 7025 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.384 |
| walker |  | 7131 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.384 |
| walker |  | 7139 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.384 |
| walker |  | 7147 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.384 |
| walker |  | 7155 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.384 |
| walker |  | 7217 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.384 |
| walker |  | 7241 | 24 | Code::CodeKey { rung: Doc, file: xlstm/components/util.py, decl: 2, sub: 0, line: 11 } |  |  | 0.384 |
| walker |  | 7257 | 16 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 11, sub: 0, line: 73 } |  |  | 0.384 |
| walker |  | 7271 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 7386 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.384 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.373 |
| walker |  | 7429 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.376 |
| walker |  | 7492 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.376 |
| walker |  | 7588 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.376 |
| walker |  | 7608 | 20 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 2, sub: 0, line: 20 } |  |  | 0.376 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.370 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.364 |
| walker |  | 7899 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.364 |
| walker |  | 7925 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.364 |
| walker |  | 7975 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.364 |
| walker |  | 8031 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.364 |
| walker |  | 8039 | 8 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 6, sub: 0, line: 95 } |  |  | 0.364 |
| walker |  | 8068 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.366 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.359 |
| walker |  | 8185 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.359 |
| walker |  | 8322 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.366 |
| walker |  | 8375 | 53 | Code::CodeKey { rung: Doc, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.368 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.362 |
| walker |  | 8434 | 59 | Code::CodeKey { rung: Body, file: xlstm/blocks/xlstm_block.py, decl: 7, sub: 0, line: 89 } |  |  | 0.363 |
| walker |  | 8463 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.366 |
| walker |  | 8550 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.366 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.378 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.394 |
| walker |  | 8807 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.394 |
| walker |  | 8860 | 53 | Code::CodeKey { rung: Doc, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.394 |
| walker |  | 8889 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.398 |
| walker |  | 8912 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.398 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.395 |
| walker |  | 8971 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.395 |
| walker |  | 8979 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.395 |
| walker |  | 9053 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.395 |
| walker |  | 9073 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.395 |
| walker |  | 9078 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.395 |
| walker |  | 9096 | 18 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 1, sub: 0, line: 7 } |  |  | 0.395 |
| walker |  | 9127 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.397 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.393 |
| walker |  | 9168 | 41 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 3, sub: 0, line: 22 } |  |  | 0.393 |
| walker |  | 9270 | 102 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 1, sub: 0, line: 9 } |  |  | 0.393 |
| walker |  | 9297 | 27 | Code::CodeKey { rung: Body, file: xlstm/blocks/mlstm/block.py, decl: 2, sub: 0, line: 17 } |  |  | 0.393 |
| walker |  | 9328 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 9366 | 38 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 3, sub: 0, line: 29 } |  |  | 0.394 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.386 |
| walker |  | 9491 | 125 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 1, sub: 0, line: 12 } |  |  | 0.386 |
| walker |  | 9560 | 69 | Code::CodeKey { rung: Body, file: xlstm/blocks/slstm/block.py, decl: 2, sub: 0, line: 21 } |  |  | 0.386 |
| walker |  | 9621 | 61 | Code::CodeKey { rung: Body, file: xlstm/components/feedforward.py, decl: 7, sub: 0, line: 72 } |  |  | 0.386 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.382 |
| walker |  | 9720 | 99 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 9735 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.385 |
| walker |  | 9764 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.387 |
| walker |  | 9779 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.387 |
| walker |  | 9796 | 17 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.387 |
| walker |  | 9816 | 20 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.387 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.385 |
| walker |  | 9977 | 161 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.385 |
| walker |  | 9996 | 19 | Code::CodeKey { rung: Body, file: xlstm/components/linear_headwise.py, decl: 5, sub: 0, line: 67 } |  |  | 0.385 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.383 |

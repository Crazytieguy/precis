Score(3000)=0.496 I=0.542 C=0.454 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.667/0.568/0.496/0.402/0.407/0.402

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 74 | 9 | Fs::DirListing { dir: notebooks } |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.589 |
| walker |  | 177 | 103 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.610 |
| walker |  | 180 | 3 | Fs::DirListing { dir: .github } |  |  | 0.610 |
| walker |  | 188 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.611 |
| walker |  | 216 | 28 | Fs::DirListing { dir: res } |  |  | 0.613 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.563 |
| walker |  | 231 | 15 | Fs::DirListing { dir: notebooks/xlstm_large } |  |  | 0.565 |
| walker |  | 267 | 36 | Fs::DirListing { dir: xlstm } |  |  | 0.577 |
| walker |  | 284 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 306 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.593 |
| walker |  | 329 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.600 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.444 |
| walker |  | 352 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.452 |
| walker |  | 374 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.452 |
| walker |  | 390 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.452 |
| walker |  | 418 | 28 | Fs::DirListing { dir: xlstm/xlstm_large } |  |  | 0.542 |
| walker |  | 453 | 35 | Fs::DirListing { dir: xlstm/components } |  |  | 0.691 |
| walker |  | 499 | 46 | Fs::DirListing { dir: experiments } |  |  | 0.695 |
| walker |  | 515 | 16 | Fs::DirListing { dir: experiments/data } |  |  | 0.697 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.629 |
| walker |  | 541 | 26 | Fs::DirListing { dir: experiments/data/formal_language } |  |  | 0.634 |
| walker |  | 569 | 28 | Fs::DirListing { dir: experiments/data/formal_language/tasks } |  |  | 0.642 |
| walker |  | 654 | 85 | Toml::Identity { file: pyproject.toml } |  |  | 0.643 |
| walker |  | 687 | 33 | Fs::DirListing { dir: notebooks/xlstm } |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 873 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.683 |
| walker |  | 928 | 55 | Fs::DirListing { dir: tests } |  |  | 0.699 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.641 |
| walker |  | 1115 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1227 | 112 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 1264 | 37 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 1, sub: 0, line: 11 } |  |  | 0.661 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.667 |
| walker |  | 1438 | 174 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.667 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.614 |
| walker |  | 1614 | 176 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 2, sub: 0, line: 17 } |  |  | 0.614 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.593 |
| walker |  | 1670 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.594 |
| walker |  | 1712 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 1743 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.566 |
| walker |  | 1853 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.566 |
| walker |  | 1909 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.566 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.522 |
| walker |  | 2216 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.526 |
| walker |  | 2222 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.526 |
| walker |  | 2382 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 2403 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.559 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.533 |
| walker |  | 2558 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.533 |
| walker |  | 2640 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.535 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.515 |
| walker |  | 2714 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.515 |
| walker |  | 2760 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 2818 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.515 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.496 |
| walker |  | 2938 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.496 |
| walker |  | 2997 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.496 |
| walker |  | 3011 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3114 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.483 |
| walker |  | 3123 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.483 |
| walker |  | 3213 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.483 |
| walker |  | 3228 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.483 |
| walker |  | 3259 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.464 |
| walker |  | 3382 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.464 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.451 |
| walker |  | 3509 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.451 |
| walker |  | 3548 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.451 |
| walker |  | 3558 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.451 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.434 |
| walker |  | 3755 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.439 |
| walker |  | 3820 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 3837 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.439 |
| walker |  | 3881 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.439 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.423 |
| walker |  | 3903 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.423 |
| walker |  | 3926 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.423 |
| walker |  | 4128 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.423 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.402 |
| walker |  | 4295 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.402 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.394 |
| walker |  | 4465 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 4495 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.410 |
| walker |  | 4525 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.410 |
| walker |  | 4565 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.410 |
| walker |  | 4607 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.412 |
| walker |  | 4655 | 48 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.412 |
| walker |  | 4705 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.413 |
| walker |  | 4747 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.414 |
| walker |  | 4807 | 60 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.415 |
| walker |  | 4852 | 45 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.416 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.402 |
| walker |  | 4930 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.408 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.396 |
| walker |  | 5310 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.399 |
| walker |  | 5350 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.399 |
| walker |  | 5403 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.399 |
| walker |  | 5412 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.399 |
| walker |  | 5421 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.399 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.417 |
| walker |  | 5496 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.417 |
| walker |  | 5504 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.417 |
| walker |  | 5518 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.417 |
| walker |  | 5534 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.417 |
| walker |  | 5550 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.417 |
| walker |  | 5639 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.415 |
| walker |  | 5708 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.415 |
| walker |  | 5799 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.415 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.406 |
| walker |  | 5932 | 133 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.407 |
| walker |  | 5960 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.407 |
| walker |  | 6014 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.407 |
| walker |  | 6062 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.407 |
| walker |  | 6068 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.407 |
| walker |  | 6076 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.407 |
| walker |  | 6084 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.407 |
| walker |  | 6151 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.407 |
| walker |  | 6159 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.407 |
| walker |  | 6167 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.407 |
| walker |  | 6244 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.407 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.394 |
| walker |  | 6259 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.394 |
| walker |  | 6276 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.394 |
| walker |  | 6293 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.394 |
| walker |  | 6310 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.394 |
| walker |  | 6352 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.397 |
| walker |  | 6458 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.397 |
| walker |  | 6466 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.397 |
| walker |  | 6474 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.397 |
| walker |  | 6482 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.397 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.390 |
| walker |  | 6544 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.390 |
| walker |  | 6558 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 6673 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.390 |
| walker |  | 6731 | 58 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 5, sub: 0, line: 49 } |  |  | 0.390 |
| walker |  | 6774 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.388 |
| walker |  | 6837 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.388 |
| walker |  | 6933 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.388 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.382 |
| walker |  | 7224 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.382 |
| walker |  | 7250 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.382 |
| walker |  | 7300 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.382 |
| walker |  | 7356 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.382 |
| walker |  | 7385 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.372 |
| walker |  | 7502 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.373 |
| walker |  | 7639 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.380 |
| walker |  | 7668 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.376 |
| walker |  | 7755 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.376 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.371 |
| walker |  | 8012 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.371 |
| walker |  | 8041 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.375 |
| walker |  | 8064 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.375 |
| walker |  | 8123 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.375 |
| walker |  | 8131 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.375 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.368 |
| walker |  | 8205 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.368 |
| walker |  | 8236 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.369 |
| walker |  | 8277 | 41 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 3, sub: 0, line: 22 } |  |  | 0.369 |
| walker |  | 8379 | 102 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 1, sub: 0, line: 9 } |  |  | 0.369 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.364 |
| walker |  | 8410 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| walker |  | 8448 | 38 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 3, sub: 0, line: 29 } |  |  | 0.365 |
| walker |  | 8573 | 125 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 1, sub: 0, line: 12 } |  |  | 0.365 |
| walker |  | 8593 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.365 |
| walker |  | 8601 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.365 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.377 |
| walker |  | 8700 | 99 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 8715 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.380 |
| walker |  | 8744 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.382 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.398 |
| walker |  | 8907 | 163 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.399 |
| walker |  | 8920 | 13 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.399 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.397 |
| walker |  | 9021 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.405 |
| walker |  | 9042 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.405 |
| walker |  | 9063 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.405 |
| walker |  | 9106 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.405 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.401 |
| walker |  | 9150 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.401 |
| walker |  | 9197 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.401 |
| walker |  | 9244 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.401 |
| walker |  | 9298 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.401 |
| walker |  | 9307 | 9 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.401 |
| walker |  | 9385 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.401 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.393 |
| walker |  | 9473 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.393 |
| walker |  | 9561 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.393 |
| walker |  | 9583 | 22 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.393 |
| walker |  | 9619 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.395 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.390 |
| walker |  | 9704 | 85 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 3, sub: 0, line: 39 } |  |  | 0.390 |
| walker |  | 9783 | 79 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 6, sub: 0, line: 127 } |  |  | 0.390 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.388 |
| walker |  | 9971 | 188 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.401 |
| walker |  | 9992 | 21 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.402 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.399 |

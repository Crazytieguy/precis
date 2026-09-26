Score(3000)=0.478 I=0.523 C=0.436 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.679/0.578/0.478/0.448/0.516/0.492

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 74 | 9 | Fs::DirListing { dir: notebooks } |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.589 |
| walker |  | 177 | 103 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.610 |
| walker |  | 205 | 28 | Fs::DirListing { dir: res } |  |  | 0.612 |
| walker |  | 220 | 15 | Fs::DirListing { dir: notebooks/xlstm_large } |  |  | 0.613 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.563 |
| walker |  | 256 | 36 | Fs::DirListing { dir: xlstm } |  |  | 0.576 |
| walker |  | 278 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.591 |
| walker |  | 301 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.597 |
| walker |  | 324 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.606 |
| walker |  | 346 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.449 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.449 |
| walker |  | 362 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.449 |
| walker |  | 390 | 28 | Fs::DirListing { dir: xlstm/xlstm_large } |  |  | 0.539 |
| walker |  | 425 | 35 | Fs::DirListing { dir: xlstm/components } |  |  | 0.687 |
| walker |  | 471 | 46 | Fs::DirListing { dir: experiments } |  |  | 0.691 |
| walker |  | 487 | 16 | Fs::DirListing { dir: experiments/data } |  |  | 0.694 |
| walker |  | 513 | 26 | Fs::DirListing { dir: experiments/data/formal_language } |  |  | 0.700 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.631 |
| walker |  | 541 | 28 | Fs::DirListing { dir: experiments/data/formal_language/tasks } |  |  | 0.639 |
| walker |  | 558 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 643 | 85 | Toml::Identity { file: pyproject.toml } |  |  | 0.640 |
| walker |  | 653 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.643 |
| walker |  | 686 | 33 | Fs::DirListing { dir: notebooks/xlstm } |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 872 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.683 |
| walker |  | 872 | 0 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.683 |
| walker |  | 927 | 55 | Fs::DirListing { dir: tests } |  |  | 0.699 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.641 |
| walker |  | 1114 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1170 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.662 |
| walker |  | 1252 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.665 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.671 |
| walker |  | 1409 | 157 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.679 |
| walker |  | 1483 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.679 |
| walker |  | 1529 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.625 |
| walker |  | 1587 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.625 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.604 |
| walker |  | 1735 | 148 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.605 |
| walker |  | 1766 | 31 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.605 |
| walker |  | 1797 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.576 |
| walker |  | 1935 | 138 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.577 |
| walker |  | 1963 | 28 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.577 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.532 |
| walker |  | 2281 | 318 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.536 |
| walker |  | 2302 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.536 |
| walker |  | 2392 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.536 |
| walker |  | 2457 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.511 |
| walker |  | 2474 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.511 |
| walker |  | 2518 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.511 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.492 |
| walker |  | 2720 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.492 |
| walker |  | 2887 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.492 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.473 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.460 |
| walker |  | 3057 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 3097 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.480 |
| walker |  | 3147 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.480 |
| walker |  | 3169 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.480 |
| walker |  | 3223 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.480 |
| walker |  | 3247 | 24 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.480 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.461 |
| walker |  | 3317 | 70 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.461 |
| walker |  | 3339 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.461 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.448 |
| walker |  | 3442 | 103 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.451 |
| walker |  | 3464 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.452 |
| walker |  | 3522 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.459 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.443 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.428 |
| walker |  | 3902 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.431 |
| walker |  | 4081 | 179 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 0, line: 30 } |  |  | 0.451 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.428 |
| walker |  | 4292 | 211 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 1, line: 30 } |  |  | 0.446 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.440 |
| walker |  | 4528 | 236 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 2, line: 30 } |  |  | 0.480 |
| walker |  | 4705 | 177 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 3, line: 30 } |  |  | 0.498 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.479 |
| walker |  | 4887 | 182 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 4, line: 30 } |  |  | 0.499 |
| walker |  | 5039 | 152 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 5, line: 30 } |  |  | 0.517 |
| walker |  | 5079 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 5150 | 71 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.517 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.501 |
| walker |  | 5233 | 83 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.501 |
| walker |  | 5247 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.501 |
| walker |  | 5255 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.501 |
| walker |  | 5269 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 5372 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.501 |
| walker |  | 5381 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.501 |
| walker |  | 5397 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.501 |
| walker |  | 5405 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.501 |
| walker |  | 5421 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.501 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.512 |
| walker |  | 5510 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 5579 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.513 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.505 |
| walker |  | 5670 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.506 |
| walker |  | 5803 | 133 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.507 |
| walker |  | 5815 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.507 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.496 |
| walker |  | 5975 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6006 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 6129 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.516 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.499 |
| walker |  | 6277 | 148 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.499 |
| walker |  | 6295 | 18 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.499 |
| walker |  | 6305 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.499 |
| walker |  | 6359 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 6434 | 75 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.499 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.489 |
| walker |  | 6517 | 83 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.489 |
| walker |  | 6665 | 148 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.489 |
| walker |  | 6670 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.489 |
| walker |  | 6675 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.489 |
| walker |  | 6680 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.489 |
| walker |  | 6685 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.489 |
| walker |  | 6714 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 6831 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.491 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.480 |
| walker |  | 6968 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.481 |
| walker |  | 6983 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.482 |
| walker |  | 7005 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.482 |
| walker |  | 7028 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.482 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.480 |
| walker |  | 7129 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 7172 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.481 |
| walker |  | 7216 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.481 |
| walker |  | 7267 | 51 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.481 |
| walker |  | 7292 | 25 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.481 |
| walker |  | 7343 | 51 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.481 |
| walker |  | 7368 | 25 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.481 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.467 |
| walker |  | 7439 | 71 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.467 |
| walker |  | 7509 | 70 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.467 |
| walker |  | 7589 | 80 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.467 |
| walker |  | 7669 | 80 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.467 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.460 |
| walker |  | 7705 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 7815 | 110 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 3, sub: 0, line: 39 } |  |  | 0.462 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.455 |
| walker |  | 7869 | 54 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 6, sub: 0, line: 127 } |  |  | 0.455 |
| walker |  | 8057 | 188 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.470 |
| walker |  | 8093 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.463 |
| walker |  | 8185 | 92 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 3, sub: 0, line: 33 } |  |  | 0.463 |
| walker |  | 8233 | 48 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.463 |
| walker |  | 8299 | 66 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 7, sub: 0, line: 123 } |  |  | 0.463 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.457 |
| walker |  | 8432 | 133 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.460 |
| walker |  | 8488 | 56 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 8502 | 14 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/utils.py, decl: 2, sub: 0, line: 10 } |  |  | 0.461 |
| walker |  | 8519 | 17 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.461 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.469 |
| walker |  | 8636 | 117 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 8651 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.477 |
| walker |  | 8680 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.481 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.492 |
| walker |  | 8825 | 145 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.493 |
| walker |  | 8853 | 28 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 8954 | 101 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.495 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.492 |
| walker |  | 8982 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.492 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.487 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.478 |
| walker |  | 9485 | 503 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 9507 | 22 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.600 |
| walker |  | 9516 | 9 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.600 |
| walker |  | 9558 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 9645 | 87 | Plaintext::DeclSurface { file: xlstm/blocks/slstm/src/cuda/slstm_pointwise.cuh } |  |  | 0.600 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.593 |
| walker |  | 9655 | 10 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.593 |
| walker |  | 9684 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/cell.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 9729 | 45 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 1, sub: 0, line: 13 } |  |  | 0.595 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.590 |
| walker |  | 9847 | 118 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 2, sub: 0, line: 20 } |  |  | 0.590 |
| walker |  | 9913 | 66 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 5, sub: 0, line: 75 } |  |  | 0.590 |
| walker |  | 9935 | 22 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.590 |
| walker |  | 9989 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.590 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.597 |
